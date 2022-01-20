use bevy::prelude::*;

use super::build_map::build_map;
use super::cursor::{create_cursor, handle_change_cursor};

use crate::awrs::resources::cursor::SelectEvent;
use crate::awrs::resources::unit::DamageEvent;
use crate::awrs::resources::{
    cursor::ChangeCursorEvent,
    state::{AppState, GameState},
};
use crate::awrs::{register_inputs::InputEvent, resources::action_event::ActionEvent};

pub struct SetupPlugin;

impl Plugin for SetupPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_event::<ActionEvent>()
            .add_event::<DamageEvent>()
            .add_event::<ChangeCursorEvent>()
            .add_event::<SelectEvent>()
            .add_event::<InputEvent>()
            .add_system(handle_change_cursor.system())
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
            .add_system_set(SystemSet::on_update(AppState::InGame(GameState::SetUp)));
    }
}

// Should probably listen for loading to be finished.
fn transition_to_browsing(mut game_state: ResMut<State<AppState>>) {
    info!("Done loading! Start Browsing!");
    game_state
        .set(AppState::InGame(GameState::Browsing))
        .expect("Problem transitioning to browsing state")
}
