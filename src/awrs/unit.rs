use bevy::prelude::*;

use super::{cell::Cell, constants::TILE_SIZE};

#[derive(Clone)]
pub struct UnitHealth(pub f32);

// Or, to avoid pub
// impl From<u32> for UnitHealth {
//     fn from(val: u32) -> UnitHealth {
//         UnitHealth(val)
//     }
// }

// impl From<UnitHealth> for u32 {
//     fn from(health: UnitHealth) -> u32 {
//         health.0
//     }
// }

#[derive(Clone)]
pub struct Team(pub u32);

// Or, to avoid pub
// impl From<u32> for Team {
//     fn from(val: u32) -> Team {
//         Team(val)
//     }
// }

// impl From<Team> for u32 {
//     fn from(team: Team) -> u32 {
//         team.0
//     }
// }

pub struct Selected;

pub struct Unit {
    pub unit_type: usize,
    pub team: Team,
    pub location: Cell,
    pub health: UnitHealth,
    // pub ammo: Ammo,
    // etc. etc..
}

#[derive(Bundle)]
pub struct UnitBundle {
    pub id: usize,
    pub data: Unit,
    #[bundle]
    pub sprite: SpriteSheetBundle,
}

// Very similar to moving cursor.
// Could have Movable struct component so that this can be reused?
// Or could extract movement logic into a separate function?
pub fn handle_unit_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut unit_query: Query<(&mut Transform, &mut Unit), With<Selected>>,
) {
    info!("Unit movement system goes brrrr");
    for (mut transform, mut unit) in unit_query.iter_mut() {
        info!("Units go brrrrr");
        if keyboard_input.just_pressed(KeyCode::W) {
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
