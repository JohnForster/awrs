use bevy::prelude::*;

use super::{plugins::*, resources::state::AppState};

pub struct AWRSPlugin;

impl Plugin for AWRSPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(LoadAssets)
            .add_state(AppState::Loading)
            // Add plugins for each of the InGame states
            .add_plugin(SetupPlugin)
            .add_plugin(InterfacePlugin)
            .add_plugin(BrowsingPlugin)
            .add_plugin(UnitMenuPlugin)
            .add_plugin(MoveUnitPlugin)
            .add_plugin(TargetingPlugin);
        // Add events
    }
}
