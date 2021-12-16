use bevy::prelude::*;

use super::{
    cell::Cell,
    cursor::Cursor,
    game::{AppState, GameState},
    map::ActiveTeam,
    unit::Unit,
};

pub fn handle_open_choose_target(mut cursor_query: Query<&mut TextureAtlasSprite, With<Cursor>>) {
    let mut texture_atlas_sprite = cursor_query.single_mut().expect("Should be a cursor");
    texture_atlas_sprite.index = 1;
}

pub fn handle_exit_choose_target(mut cursor_query: Query<&mut TextureAtlasSprite, With<Cursor>>) {
    let mut texture_atlas_sprite = cursor_query.single_mut().expect("Should be a cursor");
    texture_atlas_sprite.index = 0;
}

pub fn handle_cursor_target_select(
    keyboard_input: Res<Input<KeyCode>>,
    mut cursor_query: Query<&Cell, With<Cursor>>,
    mut units_query: Query<&mut Unit>,
    mut game_state: ResMut<State<AppState>>,
    active_team: Res<ActiveTeam>,
) {
    for cursor_cell in cursor_query.iter_mut() {
        if keyboard_input.just_pressed(KeyCode::Space) {
            for mut unit in units_query.iter_mut() {
                if unit.team == active_team.team {
                    continue;
                }

                let unit_cell = &unit.location;
                if unit_cell.x == cursor_cell.x && unit_cell.y == cursor_cell.y {
                    info!("Attacking!");
                    unit.health.0 -= 1.0;
                    info!("Unit health: {:?}", unit.health.0);
                    game_state
                        .set(AppState::InGame(GameState::Browsing))
                        .expect("Problem changing state");
                }
            }
        }
    }
}
