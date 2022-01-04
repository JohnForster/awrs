use bevy::prelude::*;

use crate::awrs::{
    choose_target::{open_target_selection, target_select},
    cursor::handle_cursor_select,
    game::{AppState, GameState},
    register_inputs::register_inputs,
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
                .with_system(register_inputs.system().label("inputs"))
                .with_system(handle_cursor_select.system().after("inputs"))
                .with_system(target_select.system().after("inputs")),
        );
    }
}
