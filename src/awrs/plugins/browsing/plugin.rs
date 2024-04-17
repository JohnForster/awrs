use bevy::prelude::*;

use super::browsing::*;

use crate::awrs::{
    resources::cursor::{handle_cursor_move, handle_cursor_select},
    resources::state::GameState,
};

pub struct BrowsingPlugin;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
struct BrowsingSet;

impl Plugin for BrowsingPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(Update, BrowsingSet.run_if(in_state(GameState::Browsing)))
            .add_systems(OnEnter(GameState::Browsing), open_browse)
            .add_systems(
                Update,
                (
                    browse_select,
                    handle_cursor_move,
                    listen_for_open_menu,
                    handle_cursor_select,
                )
                    .in_set(BrowsingSet),
            );

        // app.add_system_set(SystemSet::on_enter(browsing).with_system(open_browse))
        //     .add_system_set(
        //         SystemSet::on_update(browsing)
        //             .with_system(browse_select)
        //             .with_system(handle_cursor_move)
        //             .with_system(listen_for_open_menu)
        //             .with_system(handle_cursor_select),
        //     );
    }
}
