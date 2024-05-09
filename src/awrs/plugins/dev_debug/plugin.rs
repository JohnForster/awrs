pub struct DebugPlugin;
use bevy::prelude::*;

use crate::awrs::resources::{state::AppState, state::GameState, state::MenuState};

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, display_change_state);
    }
}

fn display_change_state(
    menu: Res<State<MenuState>>,
    game: Res<State<GameState>>,
    app: Res<State<AppState>>,
) {
    if app.is_changed() {
        info!("App state changed to {:?}", app.get());
    }
    if game.is_changed() {
        info!("Game state changed to {:?}", game.get());
    }
    if menu.is_changed() {
        info!("Menu state changed to {:?}", menu.get());
    }
}
