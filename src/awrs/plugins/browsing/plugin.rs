use bevy::prelude::*;

use super::browsing::*;

use crate::awrs::{
    register_inputs::register_inputs,
    resources::cursor::{handle_cursor_move, handle_cursor_select},
    resources::state::GameState,
};

pub struct BrowsingPlugin;

impl Plugin for BrowsingPlugin {
    fn build(&self, app: &mut AppBuilder) {
        let browsing = GameState::Browsing;
        app.add_system_set(SystemSet::on_enter(browsing).with_system(open_browse.system()))
            .add_system_set(
                SystemSet::on_update(browsing)
                    .with_system(register_inputs.system())
                    .with_system(browse_select.system())
                    .with_system(handle_cursor_move.system())
                    .with_system(handle_cursor_select.system()),
            );
    }
}
