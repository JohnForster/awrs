use bevy::prelude::*;

use super::cursor::*;
use super::load_assets::*;
use super::map::*;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    _MainMenu,
    InGame(GameState),
    Loading,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum GameState {
    _Loading,
    Running,
    _Paused,
    _UnitMenu,
    _BuildingMenu,
    _MoveUnit,
    _EnemyTurn,
}

pub struct AWRSPlugin;

impl Plugin for AWRSPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(LoadAssets)
            .add_state(AppState::Loading)
            .add_system_set(
                SystemSet::on_enter(AppState::InGame(GameState::Running))
                    .with_system(build_map.system())
                    .with_system(create_cursor.system()),
            )
            .add_system_set(
                SystemSet::on_update(AppState::InGame(GameState::Running))
                    .with_system(handle_cursor_move.system())
                    .with_system(handle_cursor_select.system()),
            );
    }
}
