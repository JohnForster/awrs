use bevy::prelude::*;

use super::unit::Team;

#[derive(Component)]
pub struct GameMap {
    pub width: usize,
    pub height: usize,
}

#[derive(Component, Bundle)]
struct GameMapBundle {
    game_map: GameMap,
    transform: Transform,
    global_transform: GlobalTransform,
}

pub struct ActiveTeam {
    pub team: Team,
}
