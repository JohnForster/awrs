use bevy::prelude::*;

use super::game_menu::{exit_game_menu, game_menu_input, open_game_menu};

use crate::awrs::resources::state::GameState;

pub struct GameMenuPlugin;

impl Plugin for GameMenuPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(
            SystemSet::on_enter(GameState::GameMenu).with_system(open_game_menu.system()),
        )
        .add_system_set(
            SystemSet::on_update(GameState::GameMenu).with_system(game_menu_input.system()),
        )
        .add_system_set(
            SystemSet::on_exit(GameState::GameMenu).with_system(exit_game_menu.system()),
        );
    }
}
