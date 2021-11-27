use bevy::prelude::*;

use super::cell::*;
use super::constants::*;
use super::sprite_loading::UIAtlas;
use super::unit::*;

pub struct Cursor;

pub fn create_cursor(mut commands: Commands, ui_atlas: Res<UIAtlas>) {
    info!("Creating Cursor");
    let x = 0;
    let y = 0;
    let starting_position = Vec3::new(x as f32, y as f32, 0.0) * TILE_SIZE;
    let adjustment = Vec3::new(4.0, -5.0, 1.0);

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
    mut cursor_query: Query<(&mut Timer, &mut Transform, &mut Cell, &Cursor)>,
) {
    for (mut _timer, mut transform, mut cell, _) in cursor_query.iter_mut() {
        // timer.tick(time.delta());

        // if !timer.finished() {
        //     continue;
        // }

        if keyboard_input.just_pressed(KeyCode::W) {
            transform.translation.y += 1.0 * TILE_SIZE;
            cell.y += 1;
        }

        if keyboard_input.just_pressed(KeyCode::A) {
            transform.translation.x -= 1.0 * TILE_SIZE;
            cell.x -= 1;
        }

        if keyboard_input.just_pressed(KeyCode::S) {
            transform.translation.y -= 1.0 * TILE_SIZE;
            cell.y -= 1;
        }

        if keyboard_input.just_pressed(KeyCode::D) {
            transform.translation.x += 1.0 * TILE_SIZE;
            cell.x += 1;
        }

        // timer.reset()
    }
}

pub fn handle_cursor_select(
    keyboard_input: Res<Input<KeyCode>>,
    mut cursor_query: Query<(&mut Transform, &Cell, &Cursor)>,
    mut units_query: Query<&Unit>,
    mut commands: Commands,
) {
    for (mut cursor_transform, cursor_cell, _) in cursor_query.iter_mut() {
        if keyboard_input.just_pressed(KeyCode::Space) {
            for unit in units_query.iter_mut() {
                let unit_cell = &unit.location;
                if unit_cell.x == cursor_cell.x && unit_cell.y == cursor_cell.y {
                    info!("Health: {:?}", unit.health.0);
                }
            }
        }
    }
}
