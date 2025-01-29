use bevy::prelude::*;

use super::{
    client::{recv_info, send_info, SendWebsocketMessageEvent},
    setup::{handle_tasks, setup_connection, WebSocketConnectionEvents},
};

pub struct WebsocketClientPlugin;

// #[derive(Debug, Clone, Eq, PartialEq, Hash, Copy, States)]
// enum Websockets {
//     Active,
//     Inactive,
// }

// ! Need to set up connection with: ev_connect.send(WebSocketConnectionEvents::SetupConnection);

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
            .add_systems(
                Update,
                (setup_connection, handle_tasks, send_info, recv_info),
            );
    }
}
