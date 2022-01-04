use bevy::prelude::*;

use crate::awrs::{
    cursor::{browse_select, handle_cursor_move, handle_cursor_select, open_browse},
    game::{AppState, GameState},
    register_inputs::register_inputs,
    unit::handle_damage,
};

pub struct BrowsingPlugin;

impl Plugin for BrowsingPlugin {
    fn build(&self, app: &mut AppBuilder) {
        let browsing = AppState::InGame(GameState::Browsing);
        app.add_system_set(SystemSet::on_enter(browsing).with_system(open_browse.system()))
            .add_system_set(
                SystemSet::on_update(browsing)
                    .with_system(register_inputs.system())
                    .with_system(browse_select.system())
                    .with_system(handle_cursor_move.system())
                    .with_system(handle_cursor_select.system())
                    .with_system(handle_damage.system()),
            );
    }
}
