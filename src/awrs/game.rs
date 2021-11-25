use bevy::prelude::*;

use super::cursor::*;
use super::map::*;

// enum AppState {
//     MainMenu,
//     InGame,
// }

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum GameState {
    Running,
    UnitMenu,
    // Paused,
    // BuildingMenu,
    // MoveUnit,
    // EnemyTurn,
}

use super::sprite_loading::*;
use super::unit_menu::*;

pub struct AWRSPlugin;

impl Plugin for AWRSPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(load_terrain_sprites.system().label("load_sprites"))
            .add_startup_system(load_unit_sprites.system().label("load_sprites"))
            .add_startup_system(load_ui_sprites.system().label("load_sprites"))
            .add_state(GameState::Running) // see if this can go after the next two method calls
            .add_system_set(
                SystemSet::on_enter(GameState::Running)
                    .with_system(build_map.system())
                    .with_system(create_cursor.system()),
            )
            .add_system_set(
                SystemSet::on_update(GameState::Running)
                    .with_system(handle_cursor_move.system())
                    .with_system(handle_cursor_select.system()),
            )
            .add_system_set(
                SystemSet::on_enter(GameState::UnitMenu).with_system(open_unit_menu.system()),
            )
            .add_system_set(
                SystemSet::on_update(GameState::UnitMenu)
                    .with_system(handle_unit_menu_navigation.system()), // .with_system(handle_unit_menu_navigation.system()),
            );
    }
}
