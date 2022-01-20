use bevy::prelude::*;

use super::{tile::Tile, unit::UnitId};

use crate::awrs::engine::UnitHp;

// Do we _need_ a wrapper here? Can the event be the enum?
pub struct ActionEvent(pub Action);

pub enum Action {
    Attack(Entity, Entity),
    Move { entity: Entity, tiles: Vec<Tile> },
    _EndTurn,
}

// Will need to add more detail once its clear what is needed from these result events.
pub enum ActionResultEvent {
    AttackResult(Vec<(UnitId, UnitHp)>), // Include ammo in this struct?
    MoveResult(Vec<Tile>),
    EndTurnResult(u32),
}
