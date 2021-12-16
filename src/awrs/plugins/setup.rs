use bevy::prelude::*;

use crate::awrs::{
    cursor::{handle_change_cursor, ChangeCursorEvent},
    unit::{AttackEvent, DamageEvent},
};

use super::super::{cursor::create_cursor, game::AppState, game::GameState, map::build_map};

pub struct SetupPlugin;

impl Plugin for SetupPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_event::<AttackEvent>()
            .add_event::<DamageEvent>()
            .add_event::<ChangeCursorEvent>()
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
