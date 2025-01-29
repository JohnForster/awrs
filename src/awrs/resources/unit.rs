use bevy::prelude::*;

use advance_craft_engine::structures::structures::StructureType as EngineStructureType;
use advance_craft_engine::units::units::UnitType as EngineUnitType;

type UnitHealth = f32;

pub type Team = u32;

#[derive(Component)]
pub struct Selected;

#[derive(Component)]
pub struct UnitId(pub u32);

#[derive(serde::Deserialize, serde::Serialize, Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum UnitType {
    Infantry,
    Zergling,
    Baneling,
    Roach,
    SiegeTank,
}

impl From<EngineUnitType> for UnitType {
    fn from(unit_type: EngineUnitType) -> Self {
        match unit_type {
            EngineUnitType::Marine => UnitType::Infantry,
            EngineUnitType::Zergling => UnitType::Zergling,
            EngineUnitType::Baneling => UnitType::Baneling,
            EngineUnitType::Roach => UnitType::Roach,
            EngineUnitType::SiegeTank => UnitType::SiegeTank,
        }
    }
}

#[derive(Component)]
pub struct StructureId(pub u32);

#[derive(serde::Deserialize, serde::Serialize, Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum StructureType {
    CommandCentre,
    Hatchery,
}

impl From<EngineStructureType> for StructureType {
    fn from(structure_type: EngineStructureType) -> Self {
        match structure_type {
            EngineStructureType::CommandCentre => StructureType::CommandCentre,
            EngineStructureType::Hatchery => StructureType::Hatchery,
        }
    }
}

#[derive(Component)]
pub struct HPIndicator;

#[derive(Event)]
pub struct DamageEvent {
    pub entity: Entity,
    pub new_hp: UnitHealth,
}
