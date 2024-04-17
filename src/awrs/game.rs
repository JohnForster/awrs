use bevy::prelude::*;

use super::{
    plugins::*,
    register_inputs::register_inputs,
    resources::state::{AppState, GameState},
};

pub struct AWRSPlugin;

#[derive(Debug, PartialEq, Eq, Hash, Clone, SystemSet)]
struct InputSet;

impl Plugin for AWRSPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(LoadAssetsPlugin)
            .insert_state(AppState::Loading)
            .insert_state(GameState::None)
            .add_systems(Update, register_inputs)
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
            ));
        // Add events
    }
}
