pub use bevy::prelude::*;

use crate::awrs::resources::{
    start_game::{GameMetadata, StartGameEvent},
    state::{AppState, GameState, MenuState},
};

use super::{
    browsing::BrowsingPlugin, cursor::CursorPlugin, game_menu::GameMenuPlugin,
    idle_animation::IdleAnimationPlugin, interface::InterfacePlugin, move_unit::MoveUnitPlugin,
    setup::SetupPlugin, targeting::TargetingPlugin, unit_menu::UnitMenuPlugin,
};

pub struct InGamePlugin;

impl Plugin for InGamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_state(GameState::None)
            .insert_state(MenuState::Closed)
            // Add plugins for each of the InGame states
            .add_plugins((
                SetupPlugin,
                InterfacePlugin,
                BrowsingPlugin,
                IdleAnimationPlugin,
                UnitMenuPlugin,
                GameMenuPlugin,
                MoveUnitPlugin,
                TargetingPlugin,
                CursorPlugin,
            ))
            .add_systems(
                Update,
                listen_for_game_start.run_if(in_state(GameState::None)),
            );
        // Add events
    }
}

fn listen_for_game_start(
    mut commands: Commands,
    mut ev_start_game: EventReader<StartGameEvent>,
    mut next_game_state: ResMut<NextState<GameState>>,
    mut next_menu_state: ResMut<NextState<MenuState>>,
    mut next_app_state: ResMut<NextState<AppState>>,
) {
    for StartGameEvent {
        scenario_state,
        online,
        game_id,
    } in ev_start_game.read()
    {
        commands.insert_resource(scenario_state.clone());
        commands.insert_resource(GameMetadata {
            _online: *online,
            _game_id: game_id.clone(),
        });
        next_app_state.set(AppState::InGame);
        next_game_state.set(GameState::SetUp);
        next_menu_state.set(MenuState::Closed);
    }
}
