use bevy::prelude::*;

use super::build_map::build_map;
use super::connect_to_server::connect_to_server;
use super::cursor::{create_cursor, handle_change_cursor};

use crate::awrs::resources::cursor::SelectEvent;
use crate::awrs::resources::unit::DamageEvent;
use crate::awrs::resources::{cursor::ChangeCursorEvent, state::GameState};
use crate::awrs::{register_inputs::InputEvent, resources::action_event::ActionEvent};

pub struct SetupPlugin;

impl Plugin for SetupPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ActionEvent>()
            .add_event::<DamageEvent>()
            .add_event::<ChangeCursorEvent>()
            .add_event::<SelectEvent>()
            .add_event::<InputEvent>()
            .add_systems(Update, handle_change_cursor)
            .add_systems(
                OnEnter(GameState::SetUp),
                (
                    build_map,
                    create_cursor,
                    transition_to_browsing,
                    connect_to_server,
                )
                    .chain(),
            );
    }
}

// Should probably listen for loading to be finished.
fn transition_to_browsing(mut next_state: ResMut<NextState<GameState>>) {
    info!("Done loading! Start Browsing!");
    next_state.set(GameState::Browsing);
}
