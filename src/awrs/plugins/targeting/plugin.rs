use bevy::prelude::*;

use super::choose_target::{open_target_selection, target_select};

use crate::awrs::resources::{
    cursor::{handle_cursor_move, handle_cursor_select},
    state::GameState,
};

pub struct TargetingPlugin;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
struct ChooseTargetSet;

impl Plugin for TargetingPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(
            Update,
            ChooseTargetSet.run_if(in_state(GameState::ChooseTarget)),
        )
        .add_systems(OnEnter(GameState::ChooseTarget), open_target_selection)
        .add_systems(
            Update,
            (handle_cursor_move, handle_cursor_select, target_select).in_set(ChooseTargetSet),
        );
        // app.add_system_set(
        //     SystemSet::on_enter(GameState::ChooseTarget).with_system(open_target_selection),
        // )
        // .add_system_set(
        //     SystemSet::on_update(GameState::ChooseTarget)
        //         .with_system(handle_cursor_move.after("inputs"))
        //         .with_system(handle_cursor_select.after("inputs"))
        //         .with_system(target_select.after("inputs").label("send action")),
        // );
    }
}
