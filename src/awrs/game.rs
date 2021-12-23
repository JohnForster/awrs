use bevy::prelude::*;

use super::load_assets::*;
use super::plugins::*;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Copy)]
pub enum AppState {
    _MainMenu,
    InGame(GameState),
    Loading,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Copy)]
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
            // Add plugins for each of the InGame states
            .add_plugin(SetupPlugin)
            .add_plugin(BrowsingPlugin)
            .add_plugin(UnitMenuPlugin)
            .add_plugin(MoveUnitPlugin)
            .add_plugin(TargetingPlugin);
    }
}
