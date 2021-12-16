use bevy::prelude::*;

use super::choose_target::*;
use super::cursor::*;
use super::load_assets::*;
use super::map::*;
use super::plugins::browsing::BrowsingPlugin;
use super::plugins::move_unit::MoveUnitPlugin;
use super::plugins::setup::SetupPlugin;
use super::plugins::targeting::TargetingPlugin;
use super::plugins::unit_menu::UnitMenuPlugin;
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
            .add_state(AppState::Loading)
            .add_plugin(SetupPlugin)
            .add_plugin(BrowsingPlugin)
            .add_plugin(UnitMenuPlugin)
            .add_plugin(MoveUnitPlugin)
            .add_plugin(TargetingPlugin);
    }
}
