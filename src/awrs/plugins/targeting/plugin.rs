use bevy::prelude::*;

use super::choose_target::{open_target_selection, target_select};

use crate::awrs::resources::{
    cursor::{handle_cursor_move, handle_cursor_select},
    state::GameState,
};

pub struct TargetingPlugin;

impl Plugin for TargetingPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(
            SystemSet::on_enter(GameState::ChooseTarget)
                .with_system(open_target_selection.system()),
        )
        .add_system_set(
            SystemSet::on_update(GameState::ChooseTarget)
                .with_system(handle_cursor_move.system().after("inputs"))
                .with_system(handle_cursor_select.system().after("inputs"))
                .with_system(target_select.system().after("inputs").label("send action")),
        );
    }
}
