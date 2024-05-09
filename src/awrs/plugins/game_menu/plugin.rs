use bevy::prelude::*;

use crate::awrs::resources::state::MenuState;

use super::game_menu::{end_turn_result, exit_game_menu, game_menu_input, open_game_menu};

pub struct GameMenuPlugin;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
struct MenuSet;

impl Plugin for GameMenuPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(Update, MenuSet.run_if(in_state(MenuState::Open)))
            .add_systems(OnEnter(MenuState::Open), open_game_menu)
            .add_systems(Update, (game_menu_input, end_turn_result).in_set(MenuSet))
            .add_systems(OnExit(MenuState::Open), exit_game_menu);

        // app.add_system_set(SystemSet::on_enter(GameState::GameMenu).with_system(open_game_menu))
        //     .add_system_set(
        //         SystemSet::on_update(GameState::GameMenu)
        //             .with_system(game_menu_input)
        //             .with_system(end_turn_result),
        //     )
        //     .add_system_set(SystemSet::on_exit(GameState::GameMenu).with_system(exit_game_menu));
    }
}
