use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Event)]
pub struct SendWebsocketMessageEvent(pub String);

#[derive(Event)]
pub struct ReceiveWebsocketMessageEvent(pub String);

impl<T: Serialize + Deserialize<'static>> From<T> for SendWebsocketMessageEvent {
    fn from(data: T) -> Self {
        let string = serde_json::to_string(&data).unwrap();
        Self(string)
    }
}

impl ReceiveWebsocketMessageEvent {
    pub fn try_into_data<'a, T: Deserialize<'a>>(&'a self) -> Result<T, serde_json::Error> {
        serde_json::from_str::<T>(&self.0)
    }
}

#[derive(Event)]
pub enum WebSocketConnectionEvents {
    SetupConnection,
}
