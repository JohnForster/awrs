pub use bevy::prelude::*;

use crate::awrs::resources::state::{GameState, MenuState};

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
            ));
        // Add events
    }
}
