use bevy::prelude::*;

use super::state::GameState;
use super::tile::Tile;

pub enum CursorStyle {
    Browse,
    Target,
    TargetSplash,
    None,
}

#[derive(Event)]
pub struct ChangeCursorEvent(pub CursorStyle);

pub struct EventContext {
    pub game_state: GameState,
}

// ? Do we need this select event, or could this be bundled into handle_cursor_select?
#[derive(Event)]
pub enum SelectEvent {
    Entity {
        entity: Entity,
        context: EventContext,
    },
    Tile {
        tile: Tile,
        context: EventContext,
    },
}
