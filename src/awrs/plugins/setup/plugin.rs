use bevy::prelude::*;

use super::build_map::build_map;
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
            .add_system(handle_change_cursor)
            .add_system_set(
                SystemSet::on_enter(GameState::SetUp)
                    .with_system(build_map.label("build map"))
                    .with_system(create_cursor.after("build map").label("create cursor"))
                    .with_system(transition_to_browsing.after("create cursor")),
            )
            .add_system_set(SystemSet::on_update(GameState::SetUp));
    }
}

// Should probably listen for loading to be finished.
fn transition_to_browsing(mut st_game: ResMut<State<GameState>>) {
    info!("Done loading! Start Browsing!");
    st_game
        .set(GameState::Browsing)
        .expect("Problem transitioning to browsing state")
}
