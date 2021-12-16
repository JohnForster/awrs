use bevy::prelude::*;

use crate::awrs::game::GameState;

use super::cell::*;
use super::constants::*;
use super::game::AppState;
use super::map::ActiveTeam;
use super::map::GameMap;
use super::sprite_loading::UIAtlas;
use super::unit::*;

pub struct Cursor;

pub fn open_browse(mut ev_change_cursor: EventWriter<ChangeCursorEvent>) {
    ev_change_cursor.send(ChangeCursorEvent(CursorStyle::Browse));
}

pub fn create_cursor(mut commands: Commands, ui_atlas: Res<UIAtlas>) {
    info!("Creating Cursor");
    let x = 0;
    let y = 0;
    let starting_position = Vec3::new(x as f32, y as f32, 0.0) * TILE_SIZE;
    let adjustment = Vec3::new(4.0, -5.0, 2.0);

    // Combine these into the Cursor struct?
    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: ui_atlas.atlas_handle.clone(),
            sprite: TextureAtlasSprite::new(0),
            transform: Transform::from_translation(starting_position + adjustment),
            ..Default::default()
        })
        .insert(Cursor)
        .insert(Cell { x, y })
        .insert(Timer::from_seconds(0.075, false));
}

pub fn handle_cursor_move(
    _time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut cursor_query: Query<(&mut Transform, &mut Cell), With<Cursor>>,
    game_map_query: Query<&GameMap>,
) {
    let game_map = game_map_query
        .single()
        .expect("Trying to move the cursor when there is no map?!");

    for (mut transform, mut cell) in cursor_query.iter_mut() {
        if keyboard_input.just_pressed(KeyCode::W) && cell.y < game_map.height {
            transform.translation.y += 1.0 * TILE_SIZE;
            cell.y += 1;
        }

        if keyboard_input.just_pressed(KeyCode::A) && cell.x > 0 {
            transform.translation.x -= 1.0 * TILE_SIZE;
            cell.x -= 1;
        }

        if keyboard_input.just_pressed(KeyCode::S) && cell.y > 0 {
            transform.translation.y -= 1.0 * TILE_SIZE;
            cell.y -= 1;
        }

        if keyboard_input.just_pressed(KeyCode::D) && cell.x < game_map.height {
            transform.translation.x += 1.0 * TILE_SIZE;
            cell.x += 1;
        }
    }
}

pub fn handle_cursor_select(
    keyboard_input: Res<Input<KeyCode>>,
    mut cursor_query: Query<(&mut Transform, &Cell, &Cursor)>,
    mut units_query: Query<(Entity, &Unit)>,
    mut game_state: ResMut<State<AppState>>,
    active_team: Res<ActiveTeam>,
    mut commands: Commands,
) {
    for (mut _cursor_transform, cursor_cell, _) in cursor_query.iter_mut() {
        if keyboard_input.just_pressed(KeyCode::Space) {
            for (entity_id, unit) in units_query.iter_mut() {
                if unit.team != active_team.team {
                    continue;
                }
                let unit_cell = &unit.location;
                if unit_cell.x == cursor_cell.x && unit_cell.y == cursor_cell.y {
                    info!("Health: {:?}", unit.health.0);

                    // Potential alternatives to this:
                    // A resource that stores an optional handle to a unit (therefore can force only one unit selected at a time)
                    // A field on the Unit struct that says whether or not the unit is selected. (Doesn't feel very ECS?)
                    commands.entity(entity_id).insert(Selected);

                    info!("Setting game state to UnitMenu");
                    game_state
                        .set(AppState::InGame(GameState::UnitMenu))
                        .expect("Problem changing state");
                }
            }
        }
    }
}
pub enum CursorStyle {
    Browse,
    Target,
    None,
}

pub struct ChangeCursorEvent(pub CursorStyle);

pub fn handle_change_cursor(
    mut ev_change_cursor: EventReader<ChangeCursorEvent>,
    mut q_cursor_sprite: Query<&mut TextureAtlasSprite, With<Cursor>>,
) {
    for ChangeCursorEvent(cursor_style) in ev_change_cursor.iter() {
        let sprite_index = match cursor_style {
            CursorStyle::Browse => 0,
            CursorStyle::Target => 1,
            CursorStyle::None => 9,
        };
        info!("Changing cursor sprite index to {:?}", sprite_index);
        let mut cursor_sprite = q_cursor_sprite.single_mut().expect("No Cursor Found?!");
        cursor_sprite.index = sprite_index;
    }
}
