use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Event)]
pub struct SendWebsocketMessageEvent(pub String);

impl<T: Serialize + Deserialize<'static>> From<T> for SendWebsocketMessageEvent {
    fn from(data: T) -> Self {
        let string = serde_json::to_string(&data).unwrap();
        Self(string)
    }
}

#[derive(Event)]
pub enum WebSocketConnectionEvents {
    SetupConnection,
}
