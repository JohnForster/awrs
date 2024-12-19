use crate::DamageTag;

use crate::units::weapon::*;

#[derive(Clone, Copy, Debug)]
pub enum StructureType {
    Hatchery,
    CommandCentre,
}

pub struct StructureStats {
    pub id: usize,
    pub name: &'static str,
    pub race: Race,
    pub max_health: f32,
    pub tags: [Option<DamageTag>; 4],
    pub base_armour: f32,
    pub weapon_one: Option<Weapon>,
}

pub enum Race {
    Zerg,
    Terran,
    _Protoss,
}

impl StructureType {
    pub const fn value(&self) -> StructureStats {
        use StructureType::*;
        match *self {
            CommandCentre => COMMAND_CENTRE,
            Hatchery => HATCHERY,
        }
    }
}

use super::super::units::DamageTag::*;
use super::Race::*;

pub const HATCHERY: StructureStats = StructureStats {
    id: 1,
    race: Zerg,
    name: "Hatchery",
    max_health: 1500.0,
    tags: [Some(Ground), Some(Armoured), Some(Structure), None],
    base_armour: 1.0,
    weapon_one: Some(ZERGLING_ATTACK),
};

pub const COMMAND_CENTRE: StructureStats = StructureStats {
    id: 1,
    race: Zerg,
    name: "Command Center",
    max_health: 1500.0,
    tags: [Some(Ground), Some(Armoured), Some(Structure), None],
    base_armour: 1.0,
    weapon_one: Some(ZERGLING_ATTACK),
};
