use bevy::prelude::*;
pub mod arrows;
pub mod movement_plan;

use super::movement_plan::{
    begin_unit_plan, confirm_move, exit_movement_plan, update_arrows, update_movement_plan,
    ConfirmMoveEvent, PlanUpdateEvent, UnitPlan,
};

use crate::awrs::resources::state::GameState;

pub struct MoveUnitPlugin;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
struct MoveUnitSet;

impl Plugin for MoveUnitPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(UnitPlan {
            range: 0,
            steps: vec![],
        })
        .add_event::<PlanUpdateEvent>()
        .add_event::<ConfirmMoveEvent>()
        .configure_sets(Update, MoveUnitSet.run_if(in_state(GameState::MoveUnit)))
        .add_systems(OnEnter(GameState::MoveUnit), begin_unit_plan)
        .add_systems(
            Update,
            (update_arrows, update_movement_plan, confirm_move)
                .in_set(MoveUnitSet)
                .chain(),
        )
        .add_systems(OnExit(GameState::MoveUnit), exit_movement_plan);

        // .add_system_set(SystemSet::on_enter(GameState::MoveUnit).with_system(begin_unit_plan))
        // .add_system_set(SystemSet::on_exit(GameState::MoveUnit).with_system(exit_movement_plan))
        // .add_system_set(
        //     SystemSet::on_update(GameState::MoveUnit)
        //         .with_system(update_arrows.before("update"))
        //         .with_system(update_movement_plan.label("update"))
        //         .with_system(confirm_move.label("send action")),
        // );
    }
}
