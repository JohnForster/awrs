use bevy::prelude::*;

use super::unit_menu::{exit_unit_menu, open_unit_menu, unit_menu_input};

use crate::awrs::resources::state::GameState;

pub struct UnitMenuPlugin;

impl Plugin for UnitMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::UnitMenu).with_system(open_unit_menu))
            .add_system_set(SystemSet::on_update(GameState::UnitMenu).with_system(unit_menu_input))
            .add_system_set(SystemSet::on_exit(GameState::UnitMenu).with_system(exit_unit_menu));
    }
}
