use bevy::prelude::Transform;

pub struct Tile {
    pub x: u32,
    pub y: u32,
}

pub const TILE_SIZE: f32 = 16.0;

impl From<Transform> for Tile {
    fn from(transform: Transform) -> Tile {
        Tile {
            x: (transform.translation.x / TILE_SIZE).round() as u32,
            y: (transform.translation.y / TILE_SIZE).round() as u32,
        }
    }
}

impl PartialEq for Tile {
    fn eq(&self, other: &Tile) -> bool {
        self.x == other.x && self.y == other.y
    }
}
