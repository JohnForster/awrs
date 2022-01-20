use bevy::prelude::*;

use super::unit::Team;

pub struct GameMap {
    pub width: usize,
    pub height: usize,
}

#[derive(Bundle)]
struct GameMapBundle {
    game_map: GameMap,
    transform: Transform,
    global_transform: GlobalTransform,
}

pub struct ActiveTeam {
    pub team: Team,
}
