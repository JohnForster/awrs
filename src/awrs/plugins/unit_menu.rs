use bevy::prelude::*;

use crate::awrs::{
    game::{AppState, GameState},
    unit_menu::{exit_unit_menu, open_unit_menu, unit_menu_input},
};

pub struct UnitMenuPlugin;

impl Plugin for UnitMenuPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(
            SystemSet::on_enter(AppState::InGame(GameState::UnitMenu))
                .with_system(open_unit_menu.system()),
        )
        .add_system_set(
            SystemSet::on_update(AppState::InGame(GameState::UnitMenu))
                .with_system(unit_menu_input.system()),
        )
        .add_system_set(
            SystemSet::on_exit(AppState::InGame(GameState::UnitMenu))
                .with_system(exit_unit_menu.system()),
        );
    }
}
