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

#[cfg(not(target_arch = "wasm32"))]
use tungstenite::{connect, http::Response, stream::MaybeTlsStream, Message, WebSocket};

use thiserror::Error;

use super::client::WebSocketClient;

#[derive(Event)]
pub enum WebSocketConnectionEvents {
    SetupConnection,
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
