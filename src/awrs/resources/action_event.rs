use bevy::prelude::*;

use super::{tile::Tile, unit::UnitId};

use advance_craft_engine::UnitHp;

// Do we _need_ a wrapper here? Can the event be the enum?
#[derive(Event)]
pub struct ActionEvent(pub Action);

#[derive(Debug)]
pub enum Action {
    Attack(Attack),
    Move { entity: Entity, tiles: Vec<Tile> },
    EndTurn,
}

impl From<(Entity, Tile)> for Attack {
    fn from((entity, tile): (Entity, Tile)) -> Attack {
        Attack::Ground(entity, tile)
    }
}

impl From<(Entity, Entity)> for Attack {
    fn from((attacker, defender): (Entity, Entity)) -> Attack {
        Attack::Unit(attacker, defender)
    }
}

#[derive(Debug)]
pub enum Attack {
    Unit(Entity, Entity),
    Ground(Entity, Tile),
}

// Will need to add more detail once its clear what is needed from these result events.
#[derive(Event)]
pub enum ActionResultEvent {
    AttackResult(Vec<(UnitId, UnitHp)>), // Include ammo in this struct?
    MoveResult(Vec<Tile>),
    EndTurnResult(u32),
}
