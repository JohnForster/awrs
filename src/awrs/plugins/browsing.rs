use bevy::prelude::*;

use crate::awrs::{
    cursor::{browse_select, move_cursor, open_browse, select_unit},
    game::{AppState, GameState},
    unit::{handle_attack, handle_damage},
};

pub struct BrowsingPlugin;

impl Plugin for BrowsingPlugin {
    fn build(&self, app: &mut AppBuilder) {
        let browsing = AppState::InGame(GameState::Browsing);
        app.add_system_set(SystemSet::on_enter(browsing).with_system(open_browse.system()))
            .add_system_set(
                SystemSet::on_update(browsing)
                    .with_system(move_cursor.system())
                    .with_system(select_unit.system())
                    .with_system(browse_select.system())
                    .with_system(handle_attack.system())
                    .with_system(handle_damage.system()),
            );
    }
}
