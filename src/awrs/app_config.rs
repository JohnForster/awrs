use bevy::ecs::system::Resource;
use uuid::Uuid;

#[derive(Resource, Debug)]
pub struct AppConfig {
    pub window_width: f32,
    pub window_height: f32,
    pub server_address: String,
    pub game_id: Option<Uuid>,
}
