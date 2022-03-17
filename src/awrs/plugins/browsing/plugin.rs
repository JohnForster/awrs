use bevy::prelude::*;

use super::browsing::*;

use crate::awrs::{
    resources::cursor::{handle_cursor_move, handle_cursor_select},
    resources::state::GameState,
};

pub struct BrowsingPlugin;

impl Plugin for BrowsingPlugin {
    fn build(&self, app: &mut App) {
        let browsing = GameState::Browsing;
        app.add_system_set(SystemSet::on_enter(browsing).with_system(open_browse))
            .add_system_set(
                SystemSet::on_update(browsing)
                    .with_system(browse_select)
                    .with_system(handle_cursor_move)
                    .with_system(listen_for_open_menu)
                    .with_system(handle_cursor_select),
            );
    }
}
