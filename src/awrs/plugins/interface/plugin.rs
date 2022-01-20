use bevy::prelude::*;

use super::interface::*;

use crate::awrs::resources::{
    action_event::{ActionEvent, ActionResultEvent},
    state::{AppState, GameState},
};

pub struct InterfacePlugin;

impl Plugin for InterfacePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_event::<ActionEvent>()
            .add_event::<ActionResultEvent>()
            // ! There has to be a better way! Split AppState & GameState?
            .add_system_set(
                SystemSet::on_update(AppState::InGame(GameState::Browsing))
                    .with_system(handle_action.system()),
            )
            .add_system_set(
                SystemSet::on_update(AppState::InGame(GameState::MoveUnit))
                    .with_system(handle_action.system()),
            )
            .add_system_set(
                SystemSet::on_update(AppState::InGame(GameState::ChooseTarget))
                    .with_system(handle_action.system()),
            )
            .add_system_set(
                SystemSet::on_update(AppState::InGame(GameState::UnitMenu))
                    .with_system(handle_action.system()),
            );
    }
}
