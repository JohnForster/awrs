use bevy::prelude::*;

use super::cursor::*;
use super::load_assets::*;
use super::map::*;
use super::unit::*;
use super::unit_menu::*;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    _MainMenu,
    InGame(GameState),
    Loading,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum GameState {
    _Loading,
    Browsing,
    _Paused,
    UnitMenu,
    _BuildingMenu,
    MoveUnit,
    _EnemyTurn,
}

pub struct AWRSPlugin;

impl Plugin for AWRSPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(LoadAssets)
            .add_state(AppState::Loading)
            // Browsing
            .add_system_set(
                SystemSet::on_enter(AppState::InGame(GameState::Browsing))
                    .with_system(build_map.system())
                    .with_system(create_cursor.system()),
            )
            .add_system_set(
                SystemSet::on_update(AppState::InGame(GameState::Browsing))
                    .with_system(handle_cursor_move.system())
                    .with_system(handle_cursor_select.system()),
            )
            // Unit Menu
            .add_system_set(
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
            )
            // Unit Movement
            .add_system_set(
                SystemSet::on_update(AppState::InGame(GameState::MoveUnit))
                    .with_system(handle_unit_movement.system()),
            );
    }
}
