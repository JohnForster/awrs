use bevy::prelude::*;

use crate::awrs::plugins::client::WebSocketConnectionEvents;

pub fn connect_to_server(mut ev_ws_connect: EventWriter<WebSocketConnectionEvents>) {
    ev_ws_connect.send(WebSocketConnectionEvents::SetupConnection {});
}
