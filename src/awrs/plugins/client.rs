use std::{io::ErrorKind, net::TcpStream};

#[cfg(not(target_arch = "wasm32"))]
use tungstenite::{connect, http::Response, stream::MaybeTlsStream, Message, WebSocket};

use thiserror::Error;

use bevy::{
    ecs::world::CommandQueue,
    prelude::*,
    tasks::{block_on, futures_lite::future, AsyncComputeTaskPool, Task},
};

use crate::awrs::resources::client::{
    ReceiveWebsocketMessageEvent, SendWebsocketMessageEvent, WebSocketConnectionEvents,
};

pub struct WebsocketClientPlugin;

impl Plugin for WebsocketClientPlugin {
    fn build(&self, app: &mut App) {
        #[cfg(not(target_arch = "wasm32"))]
        {
            rustls::crypto::aws_lc_rs::default_provider()
                .install_default()
                .expect("Failed to install rustls crypto provider");
        }

        app.add_event::<WebSocketConnectionEvents>()
            .add_event::<SendWebsocketMessageEvent>()
            .add_event::<ReceiveWebsocketMessageEvent>()
            .add_systems(
                Update,
                (setup_connection, handle_tasks, send_info, recv_info),
            );
    }
}

#[derive(Component)]
pub struct WebSocketClient(
    #[cfg(target_arch = "wasm32")] send_wrapper::SendWrapper<wasm_websocket::Client>,
    #[cfg(not(target_arch = "wasm32"))]
    pub  (
        WebSocket<MaybeTlsStream<TcpStream>>,
        Response<Option<Vec<u8>>>,
    ),
);

// TODO - Introduce rate limiting to prevent server message spamming.
pub fn send_info(
    mut ev_ws_message: EventReader<SendWebsocketMessageEvent>,
    mut entities_with_client: Query<(&mut WebSocketClient,)>,
) {
    for (mut client,) in entities_with_client.iter_mut() {
        for data in ev_ws_message.read() {
            info!("Sending message...");
            let msg = data.0.clone();

            #[cfg(not(target_arch = "wasm32"))]
            {
                match client.0 .0.send(Message::Binary(msg.into())) {
                    Ok(_) => info!("Data successfully sent!"),
                    #[cfg(not(target_arch = "wasm32"))]
                    Err(tungstenite::Error::Io(e)) if e.kind() == ErrorKind::WouldBlock => { /* ignore */
                    }
                    Err(e) => {
                        warn!("Could not send the message: {e:?}");
                    }
                }
            }

            #[cfg(target_arch = "wasm32")]
            {
                // TODO: do some handling so we know whether the websocket is connected yet
                let _ = client.0.socket.send_with_u8_array(msg.as_slice()); // ignore the error because the websocket may still be connecting
            }
        }
    }
}

pub fn recv_info(
    mut q: Query<(&mut WebSocketClient,)>,
    mut ev_ws_msg_recv: EventWriter<ReceiveWebsocketMessageEvent>,
) {
    for (mut client,) in q.iter_mut() {
        #[cfg(not(target_arch = "wasm32"))]
        {
            match client.0 .0.read() {
                Ok(m) => {
                    let data_string = m.to_string();
                    info!("Received: {data_string}");
                    ev_ws_msg_recv.send(ReceiveWebsocketMessageEvent(data_string));
                }
                Err(tungstenite::Error::Io(e)) if e.kind() == ErrorKind::WouldBlock => { /* ignore */
                }
                Err(e) => warn!("error receiving: {e}"),
            };
        }

        #[cfg(target_arch = "wasm32")]
        {
            while let Some(m) = client.0.recv_queue.borrow_mut().pop_front() {
                info!("Received message {m:?}")
            }
        }
    }
}

#[derive(Component)]
pub struct WebSocketConnectionSetupTask(
    #[allow(unused)] Task<Result<CommandQueue, ConnectionSetupError>>,
);

#[derive(Error, Debug)]
enum ConnectionSetupError {
    #[error("IO")]
    Io(#[from] std::io::Error),
    #[cfg(target_arch = "wasm32")]
    #[error("WebSocket")]
    WebSocket(), // TODO: remove or fill in actual error and do error handling with it?
    #[cfg(not(target_arch = "wasm32"))]
    #[error("WebSocket")]
    WebSocket(#[from] tungstenite::Error),
}

/// This function listens for WebsocketConnectionEvents, and if found, creates a
/// task to connect to the websocket server, and adds that task as an entity to
/// the world. When that task is complete, it is handled by handle_tasks
/// below.
pub fn setup_connection(
    mut ev_connect: EventReader<WebSocketConnectionEvents>,
    mut commands: Commands,
) {
    for ev in ev_connect.read() {
        match ev {
            WebSocketConnectionEvents::SetupConnection => {
                info!("Setting up connection!");
                let url = "ws://127.0.0.1:8080/";
                let entity = commands.spawn_empty().id();

                #[cfg(not(target_arch = "wasm32"))]
                {
                    let pool = AsyncComputeTaskPool::get();
                    let task = pool.spawn(async move {
                        info!("Attempting to connect to server");
                        let mut client = connect(url)?;
                        match client.0.get_mut() {
                            MaybeTlsStream::Plain(p) => p.set_nonblocking(true)?,
                            MaybeTlsStream::Rustls(stream_owned) => {
                                stream_owned.get_mut().set_nonblocking(true)?
                            }
                            _ => todo!(),
                        };
                        info!("Connected to server successfully!");
                        let mut command_queue = CommandQueue::default();

                        command_queue.push(move |world: &mut World| {
                            world
                                .entity_mut(entity)
                                .insert(WebSocketClient(client))
                                // Task is complete, so remove task component from entity
                                .remove::<WebSocketConnectionSetupTask>();
                        });

                        Ok(command_queue)
                    });
                    commands
                        .entity(entity)
                        .insert(WebSocketConnectionSetupTask(task));
                }

                #[cfg(target_arch = "wasm32")]
                {
                    commands
                        .entity(entity)
                        .insert(WebSocketClient(wasm_websocket::Client::new(url)));
                }
            }
        }
    }
}

/// This function polls `WebSocketConnectionSetupTask`s, and when they are
/// complete, it appends the cleanup commands to the world.
pub fn handle_tasks(
    mut commands: Commands,
    mut transform_tasks: Query<&mut WebSocketConnectionSetupTask>,
) {
    for mut task in &mut transform_tasks {
        if let Some(result) = block_on(future::poll_once(&mut task.0)) {
            // append the returned command queue to have it execute later
            match result {
                Ok(mut commands_queue) => {
                    commands.append(&mut commands_queue);
                }
                Err(e) => {
                    info!("Connection failed with: {e:?}");
                }
            }
        }
    }
}

#[cfg(target_arch = "wasm32")]
mod wasm_websocket {
    use std::{cell::RefCell, collections::VecDeque, rc::Rc};

    use bevy::log::info;
    use web_sys::{
        js_sys::{ArrayBuffer, Uint8Array},
        wasm_bindgen::{prelude::Closure, JsCast},
        BinaryType, Event, MessageEvent,
    };

    pub struct Client {
        pub socket: web_sys::WebSocket,
        pub recv_queue: Rc<RefCell<VecDeque<Vec<u8>>>>,
        _open_cb: Closure<dyn FnMut(Event)>,
        _message_cb: Closure<dyn FnMut(MessageEvent)>,
    }

    impl Client {
        pub fn new(url: &str) -> send_wrapper::SendWrapper<Self> {
            info!("Opening wasm websocket");
            let recv_queue = Rc::new(RefCell::new(VecDeque::new()));
            let socket = web_sys::WebSocket::new(url).expect("Failed to create WebSocket object");
            socket.set_binary_type(BinaryType::Arraybuffer);
            let open_cb: Closure<dyn FnMut(_)> = Closure::new(|_event: Event| {
                web_sys::console::log_1(&"Connection opened".into());
            });
            socket
                .add_event_listener_with_callback("open", open_cb.as_ref().dyn_ref().unwrap())
                .unwrap();
            let message_cb: Closure<dyn FnMut(_)> = Closure::new({
                let recv_queue = Rc::clone(&recv_queue);
                move |event: MessageEvent| {
                    web_sys::console::log_1(&format!("Got message: {:?}", event.data()).into());
                    if let Some(buf) = event.data().dyn_ref::<ArrayBuffer>() {
                        recv_queue
                            .borrow_mut()
                            .push_back(Uint8Array::new(buf).to_vec());
                    }
                }
            });
            socket
                .add_event_listener_with_callback("message", message_cb.as_ref().dyn_ref().unwrap())
                .unwrap();
            send_wrapper::SendWrapper::new(Client {
                socket,
                recv_queue,
                _open_cb: open_cb,
                _message_cb: message_cb,
            })
        }
    }
}
