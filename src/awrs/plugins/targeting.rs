use bevy::prelude::*;

use crate::awrs::{
    choose_target::{open_target_selection, target_select},
    cursor::{move_cursor, select_unit},
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
                .with_system(move_cursor.system())
                .with_system(select_unit.system())
                .with_system(target_select.system()),
        );
    }
}
