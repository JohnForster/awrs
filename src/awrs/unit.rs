use bevy::prelude::*;

use super::cell::*;

#[derive(Clone)]
pub struct Unit {
    unit_type: UnitType,
    team: Team,
    pub location: Cell,
    health: UnitHealth, // etc. etc..
}

#[derive(Clone)]
pub struct UnitHealth(u32);

#[derive(Clone)]
pub enum UnitType {
    Infantry,
}

#[derive(Clone)]
pub struct Team(u32);

#[derive(Bundle)]
pub struct UnitBundle {
    id: usize,
    data: Unit,
    #[bundle]
    sprite: SpriteSheetBundle,
}
