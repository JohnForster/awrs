use bevy::ecs::system::Resource;
use uuid::Uuid;

#[derive(Resource, Debug)]
pub struct AppConfig {
    pub _window_width: f32,
    pub _window_height: f32,
    pub _server_address: String,
    pub game_id: Option<Uuid>,
}
