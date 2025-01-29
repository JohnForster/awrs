use bevy::prelude::*;

use crate::awrs::resources::client::WebSocketConnectionEvents;

pub fn connect_to_server(mut ev_ws_connect: EventWriter<WebSocketConnectionEvents>) {
    ev_ws_connect.send(WebSocketConnectionEvents::SetupConnection {});
}
