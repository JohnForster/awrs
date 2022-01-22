use bevy::prelude::*;

use super::movement_plan::{
    begin_unit_plan, confirm_move, exit_movement_plan, update_arrows, update_movement_plan,
    ConfirmMoveEvent, PlanUpdateEvent, UnitPlan,
};

use crate::awrs::{register_inputs::register_inputs, resources::state::GameState};

pub struct MoveUnitPlugin;

impl Plugin for MoveUnitPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.insert_resource(UnitPlan {
            range: 0,
            steps: vec![],
        })
        .add_event::<PlanUpdateEvent>()
        .add_event::<ConfirmMoveEvent>()
        .add_system_set(
            SystemSet::on_enter(GameState::MoveUnit).with_system(begin_unit_plan.system()),
        )
        .add_system_set(
            SystemSet::on_exit(GameState::MoveUnit).with_system(exit_movement_plan.system()),
        )
        .add_system_set(
            SystemSet::on_update(GameState::MoveUnit)
                .with_system(register_inputs.system())
                .with_system(update_arrows.system().before("update"))
                .with_system(update_movement_plan.system().label("update"))
                .with_system(confirm_move.system().label("send action")),
        );
    }
}
