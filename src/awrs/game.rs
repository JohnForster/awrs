use bevy::prelude::*;

use super::{
    plugins::*,
    register_inputs::register_inputs,
    resources::state::{AppState, GameState, MenuState},
};

pub struct AWRSPlugin;

impl Plugin for AWRSPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(LoadAssetsPlugin)
            .insert_state(AppState::Loading)
            .insert_state(GameState::None)
            .insert_state(MenuState::Closed)
            .add_systems(Update, register_inputs)
            // Add plugins for each of the InGame states
            .add_plugins((
                DebugPlugin,
                SetupPlugin,
                InterfacePlugin,
                BrowsingPlugin,
                IdleAnimationPlugin,
                UnitMenuPlugin,
                GameMenuPlugin,
                MoveUnitPlugin,
                TargetingPlugin,
                WebsocketClientPlugin,
            ));
        // Add events
    }
}
