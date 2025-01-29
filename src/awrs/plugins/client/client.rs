use std::{
    io::ErrorKind,
    net::TcpStream,
    sync::{Arc, Mutex},
    time::Duration,
};

use bevy::{
    ecs::world::CommandQueue,
    prelude::*,
    tasks::{block_on, futures_lite::future, AsyncComputeTaskPool, Task},
};

use serde::{Deserialize, Serialize};
#[cfg(not(target_arch = "wasm32"))]
use tungstenite::{connect, http::Response, stream::MaybeTlsStream, Message, WebSocket};

use thiserror::Error;

#[derive(Component)]
pub struct WebSocketClient(
    #[cfg(target_arch = "wasm32")] send_wrapper::SendWrapper<wasm_websocket::Client>,
    #[cfg(not(target_arch = "wasm32"))]
    pub  (
        WebSocket<MaybeTlsStream<TcpStream>>,
        Response<Option<Vec<u8>>>,
    ),
);

#[derive(Event)]
pub struct SendWebsocketMessageEvent(String);

impl<T: Serialize + Deserialize<'static>> From<T> for SendWebsocketMessageEvent {
    fn from(data: T) -> Self {
        let string = serde_json::to_string(&data).unwrap();
        Self(string)
    }
}

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

pub fn recv_info(mut q: Query<(&mut WebSocketClient,)>) {
    for (mut client,) in q.iter_mut() {
        #[cfg(not(target_arch = "wasm32"))]
        {
            match client.0 .0.read() {
                Ok(m) => info!("Received message {m:?}"),
                Err(tungstenite::Error::Io(e)) if e.kind() == ErrorKind::WouldBlock => { /* ignore */
                }
                Err(e) => warn!("error receiving: {e}"),
            }
        }

        #[cfg(target_arch = "wasm32")]
        {
            while let Some(m) = client.0.recv_queue.borrow_mut().pop_front() {
                info!("Received message {m:?}")
            }
        }
    }
}
