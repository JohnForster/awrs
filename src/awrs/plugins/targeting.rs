use bevy::prelude::*;

use crate::awrs::{
    choose_target::{open_target_selection, select_target},
    cursor::handle_cursor_move,
    game::{AppState, GameState},
};

pub struct TargetingPlugin;

impl Plugin for TargetingPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(
            SystemSet::on_enter(AppState::InGame(GameState::ChooseTarget))
                .with_system(open_target_selection.system()),
        )
        .add_system_set(
            SystemSet::on_update(AppState::InGame(GameState::ChooseTarget))
                .with_system(handle_cursor_move.system())
                .with_system(select_target.system()),
        );
    }
}
