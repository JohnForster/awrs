use bevy::prelude::*;

use super::choose_target::*;
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
    SetUp,
    Browsing,
    _Paused,
    UnitMenu,
    _BuildingMenu,
    MoveUnit,
    ChooseTarget,
    _EnemyTurn,
}

pub struct AWRSPlugin;

impl Plugin for AWRSPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(LoadAssets)
            .add_event::<AttackEvent>()
            .add_event::<DamageEvent>()
            .add_state(AppState::Loading)
            // ------------------------ Loading ------------------------
            .add_system_set(
                SystemSet::on_enter(AppState::InGame(GameState::SetUp))
                    .with_system(build_map.system().label("build map"))
                    .with_system(
                        create_cursor
                            .system()
                            .after("build map")
                            .label("create cursor"),
                    )
                    .with_system(transition_to_browsing.system().after("create cursor")),
            )
            .add_system_set(SystemSet::on_update(AppState::InGame(GameState::SetUp)))
            // ------------------------ Browsing ------------------------
            .add_system_set(SystemSet::on_enter(AppState::InGame(GameState::Browsing)))
            .add_system_set(
                SystemSet::on_update(AppState::InGame(GameState::Browsing))
                    .with_system(handle_cursor_move.system())
                    .with_system(handle_cursor_select.system())
                    .with_system(handle_attack.system())
                    .with_system(handle_damage.system()),
            )
            // ------------------------ Unit Menu ------------------------
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
            // ------------------------ Unit Movement ------------------------
            .add_system_set(
                SystemSet::on_update(AppState::InGame(GameState::MoveUnit))
                    .with_system(handle_unit_movement.system()),
            )
            // ------------------------ Choose Target ------------------------
            .add_system_set(
                SystemSet::on_enter(AppState::InGame(GameState::ChooseTarget))
                    .with_system(handle_open_choose_target.system()),
            )
            .add_system_set(
                SystemSet::on_update(AppState::InGame(GameState::ChooseTarget))
                    .with_system(handle_cursor_move.system())
                    .with_system(handle_cursor_target_select.system()),
            )
            .add_system_set(
                SystemSet::on_exit(AppState::InGame(GameState::ChooseTarget))
                    .with_system(handle_exit_choose_target.system()),
            );
    }
}

// Should probably listen for loading to be finished.
fn transition_to_browsing(mut game_state: ResMut<State<AppState>>) {
    info!("Done loading! Start Browsing!");
    game_state
        .set(AppState::InGame(GameState::Browsing))
        .expect("Problem transitioning to browsing state")
}
