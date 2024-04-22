use bevy::prelude::*;

use super::interface::*;

use crate::awrs::{
    plugins::movement_plan::move_result,
    resources::{
        action_event::{ActionEvent, ActionResultEvent},
        state::AppState,
        unit::{handle_attack_result, handle_damage},
    },
};

pub struct InterfacePlugin;

#[derive(Debug, PartialEq, Eq, Hash, Clone, SystemSet)]
struct InputSet;

impl Plugin for InterfacePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ActionEvent>()
            .add_event::<ActionResultEvent>()
            .configure_sets(Update, InputSet.run_if(in_state(AppState::InGame)))
            .add_systems(Update, (handle_action.in_set(InputSet),))
            .add_systems(
                Update,
                (handle_attack_result, handle_damage, move_result)
                    .in_set(InputSet)
                    .after(handle_action),
            );
        // ! There has to be a better way! Split AppState & GameState?
        // .add_system_set(
        //     SystemSet::on_update(AppState::InGame)
        //         .with_system(handle_action.in_set(InputSet).after("send action"))
        //         .with_system(handle_attack_result.after(InputSet))
        //         .with_system(handle_damage.after(InputSet))
        //         .with_system(move_result.after(InputSet)),
        // );
    }
}
