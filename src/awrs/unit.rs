use bevy::prelude::*;

use super::cell::Cell;

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
