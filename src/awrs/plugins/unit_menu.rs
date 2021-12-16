use bevy::prelude::*;

use crate::awrs::{
    game::{AppState, GameState},
    unit_menu::{handle_exit_unit_menu, handle_navigate_unit_menu, handle_open_unit_menu},
};

pub struct UnitMenuPlugin;

impl Plugin for UnitMenuPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(
            SystemSet::on_enter(AppState::InGame(GameState::UnitMenu))
                .with_system(handle_open_unit_menu.system()),
        )
        .add_system_set(
            SystemSet::on_update(AppState::InGame(GameState::UnitMenu))
                .with_system(handle_navigate_unit_menu.system()),
        )
        .add_system_set(
            SystemSet::on_exit(AppState::InGame(GameState::UnitMenu))
                .with_system(handle_exit_unit_menu.system()),
        );
    }
}
