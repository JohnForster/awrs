use bevy::prelude::*;

use crate::awrs::{
    cursor::handle_cursor_move,
    game::{AppState, GameState},
    unit::{move_unit, open_move_unit},
};

pub struct MoveUnitPlugin;

impl Plugin for MoveUnitPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(
            SystemSet::on_enter(AppState::InGame(GameState::MoveUnit))
                .with_system(open_move_unit.system()),
        )
        .add_system_set(
            SystemSet::on_update(AppState::InGame(GameState::MoveUnit))
                .with_system(move_unit.system())
                .with_system(handle_cursor_move.system()),
        );
    }
}
