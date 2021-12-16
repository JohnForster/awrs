use bevy::prelude::*;

use crate::awrs::{
    cursor::{handle_cursor_move, handle_cursor_select, open_browse},
    game::{AppState, GameState},
    unit::{handle_attack, handle_damage},
};

pub struct BrowsingPlugin;

impl Plugin for BrowsingPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(
            SystemSet::on_enter(AppState::InGame(GameState::Browsing))
                .with_system(open_browse.system()),
        )
        .add_system_set(
            SystemSet::on_update(AppState::InGame(GameState::Browsing))
                .with_system(handle_cursor_move.system())
                .with_system(handle_cursor_select.system())
                .with_system(handle_attack.system())
                .with_system(handle_damage.system()),
        );
    }
}
