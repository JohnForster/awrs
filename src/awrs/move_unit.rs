use super::cell::*;
use bevy::prelude::*;

use super::constants::*;
use super::unit::*;

pub fn handle_unit_movement(
    mut unit_query: Query<(&mut Unit, &mut Transform)>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    info!("Handling Movement!");

    for (mut unit, mut transform) in unit_query.iter_mut() {
        info!("Unit!");
        if unit.selected.0 {
            info!("Found unit!");
            if keyboard_input.just_pressed(KeyCode::W) {
                info!("Pressed W!");
                transform.translation.y += 1.0 * TILE_SIZE;
                unit.location.y += 1;
            }

            if keyboard_input.just_pressed(KeyCode::A) {
                transform.translation.x -= 1.0 * TILE_SIZE;
                unit.location.x -= 1;
            }

            if keyboard_input.just_pressed(KeyCode::S) {
                transform.translation.y -= 1.0 * TILE_SIZE;
                unit.location.y -= 1;
            }

            if keyboard_input.just_pressed(KeyCode::D) {
                transform.translation.x += 1.0 * TILE_SIZE;
                unit.location.x += 1;
            }
        }
    }
}
