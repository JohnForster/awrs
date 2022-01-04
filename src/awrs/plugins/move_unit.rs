use bevy::prelude::*;

use crate::awrs::{
    game::{AppState, GameState},
    register_inputs::register_inputs,
    tile::Tile,
    unit::add_move_step,
};

pub struct MoveUnitPlugin;

pub struct UnitMove {
    pub tiles: Vec<Tile>,
    pub entities: Vec<Entity>,
}

impl Plugin for MoveUnitPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.insert_resource(UnitMove {
            tiles: vec![],
            entities: vec![],
        })
        .add_system_set(SystemSet::on_enter(AppState::InGame(GameState::MoveUnit)))
        .add_system_set(
            SystemSet::on_update(AppState::InGame(GameState::MoveUnit))
                .with_system(register_inputs.system())
                .with_system(add_move_step.system()),
        );
    }
}
