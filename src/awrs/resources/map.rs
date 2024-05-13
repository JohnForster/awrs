use bevy::prelude::*;

use super::unit::Team;

#[derive(Component)]
pub struct GameMap {
    pub width: usize,
    pub height: usize,
}

#[derive(Bundle)]
struct GameMapBundle {
    game_map: GameMap,
    transform: Transform,
    global_transform: GlobalTransform,
    inherited_visibility: InheritedVisibility,
}

#[derive(Resource, Debug)]
pub struct ActiveTeam {
    pub team: Team,
}
