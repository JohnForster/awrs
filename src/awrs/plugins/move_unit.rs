use bevy::prelude::*;

use crate::awrs::{
    game::{AppState, GameState},
    movement_plan::{
        begin_unit_plan, confirm_move, exit_movement_plan, move_result, update_arrows,
        update_movement_plan, ConfirmMoveEvent, PlanUpdateEvent, UnitPlan,
    },
    register_inputs::register_inputs,
};

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
            SystemSet::on_enter(AppState::InGame(GameState::MoveUnit))
                .with_system(begin_unit_plan.system()),
        )
        .add_system_set(
            SystemSet::on_exit(AppState::InGame(GameState::MoveUnit))
                .with_system(exit_movement_plan.system()),
        )
        .add_system_set(
            SystemSet::on_update(AppState::InGame(GameState::MoveUnit))
                .with_system(register_inputs.system())
                .with_system(update_movement_plan.system().label("update"))
                .with_system(update_arrows.system().before("update"))
                .with_system(confirm_move.system())
                .with_system(move_result.system()),
        );
    }
}
