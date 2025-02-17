use bevy::prelude::*;

use advance_craft_engine::structures::structures::StructureType as EngineStructureType;
use advance_craft_engine::units::units::UnitType as EngineUnitType;

use crate::awrs::plugins::interface::interface::ScenarioState;

use super::action_event::ActionResultEvent;

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

pub fn handle_attack_result(
    mut q_units: Query<(Entity, &UnitId, &mut Sprite)>,
    mut ev_action_result: EventReader<ActionResultEvent>,
    mut ev_damage: EventWriter<DamageEvent>,
    scenario_state: Res<ScenarioState>,
) {
    for action_result in ev_action_result.read() {
        if let ActionResultEvent::AttackResult(damaged_units) = action_result {
            for (id, hp) in damaged_units {
                for (entity, unit_id, _) in q_units.iter_mut() {
                    if unit_id.0 == id.0 {
                        info!("Sending DamageEvent");
                        ev_damage.send(DamageEvent {
                            entity,
                            new_hp: *hp,
                        });
                    }
                }
            }
        }

        const GRAY: Srgba = bevy::color::palettes::css::GRAY;
        for (_, UnitId(unit_id), mut sprite) in q_units.iter_mut() {
            if scenario_state.unit_cannot_act(unit_id) {
                sprite.color = GRAY.into();
            }
        }
    }
}

#[derive(Event)]
pub struct DamageEvent {
    entity: Entity,
    new_hp: UnitHealth,
}

// Belongs elsewhere
pub fn handle_damage(
    mut ev_damage: EventReader<DamageEvent>,
    mut units_query: Query<(&UnitId, &Children)>,
    mut q_hp_indicator: Query<(&mut Sprite, &mut Visibility), With<HPIndicator>>,
    mut commands: Commands,
    scenario_state: Res<ScenarioState>,
) {
    for DamageEvent { entity, new_hp } in ev_damage.read() {
        info!("Handling Damage Event");
        let (unit_id, children) = units_query
            .get_mut(*entity)
            .expect("Could not find unit to damage");

        for &child in children.iter() {
            let Ok((mut sprite, mut visibility)) = q_hp_indicator.get_mut(child) else {
                continue;
            };
            info!("Updating health indicator");
            match scenario_state.get_unit(unit_id.0) {
                Some(unit) => {
                    let max_health = unit.unit_type.value().max_health;
                    println!("max_health: {:?}", max_health);
                    let health_percent = new_hp / max_health;
                    let ceil_health = (health_percent * 10.0).ceil().max(0.0) as usize;
                    info!("new_hp: {:?}, ceil_health: {:?}", new_hp, ceil_health);
                    let Some(atlas) = &mut sprite.texture_atlas else {
                        continue;
                    };
                    if ceil_health == 0 {
                        commands.entity(*entity).despawn_recursive()
                    } else if ceil_health < 10 {
                        *visibility = Visibility::Visible;
                        atlas.index = ceil_health - 1;
                    }
                }
                None => commands.entity(*entity).despawn_recursive(),
            }
        }
    }
}
