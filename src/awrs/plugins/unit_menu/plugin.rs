use bevy::prelude::*;

use super::unit_menu::{exit_unit_menu, open_unit_menu, unit_menu_input};

use crate::awrs::resources::state::GameState;

pub struct UnitMenuPlugin;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
struct UnitMenuSet;

impl Plugin for UnitMenuPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(Update, UnitMenuSet.run_if(in_state(GameState::UnitMenu)))
            .add_systems(OnEnter(GameState::UnitMenu), open_unit_menu)
            .add_systems(Update, unit_menu_input.in_set(UnitMenuSet))
            .add_systems(OnExit(GameState::UnitMenu), exit_unit_menu);

        // app.add_system_set(SystemSet::on_enter(GameState::UnitMenu).with_system(open_unit_menu))
        //     .add_system_set(SystemSet::on_update(GameState::UnitMenu).with_system(unit_menu_input))
        //     .add_system_set(SystemSet::on_exit(GameState::UnitMenu).with_system(exit_unit_menu));
    }
}
