use bevy::prelude::*;

use crate::awrs::engine::{ScenarioState, UnitType as EngineUnitType};

use super::action_event::ActionResultEvent;

type UnitHealth = f32;

pub type Team = u32;

pub struct Selected;

pub struct UnitId(pub u32);

#[derive(serde::Deserialize, serde::Serialize, Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum UnitType {
    Infantry,
    Zergling,
    Baneling,
    Roach,
}

impl From<EngineUnitType> for UnitType {
    fn from(unit_type: EngineUnitType) -> Self {
        match unit_type {
            EngineUnitType::Infantry => UnitType::Infantry,
            EngineUnitType::Zergling => UnitType::Zergling,
            EngineUnitType::Baneling => UnitType::Baneling,
            EngineUnitType::Roach => UnitType::Roach,
        }
    }
}

pub struct HealthIndicator;

pub fn handle_attack_result(
    mut q_units: Query<(Entity, &UnitId, &mut TextureAtlasSprite)>,
    mut ev_action_result: EventReader<ActionResultEvent>,
    mut ev_damage: EventWriter<DamageEvent>,
    scenario_state: Res<ScenarioState>,
) {
    for action_result in ev_action_result.iter() {
        if let ActionResultEvent::AttackResult(damaged_units) = action_result {
            for (id, hp) in damaged_units {
                for (entity, unit_id, _) in q_units.iter_mut() {
                    if unit_id.0 == id.0 {
                        info!("Sending DamageEvent");
                        ev_damage.send(DamageEvent {
                            entity,
                            new_hp: *hp,
                        })
                    }
                }
            }
        }

        for (_, UnitId(unit_id), mut sprite) in q_units.iter_mut() {
            if scenario_state.unit_cannot_act(unit_id) {
                sprite.color = Color::GRAY;
            }
        }
    }
}

pub struct DamageEvent {
    entity: Entity,
    new_hp: UnitHealth,
}

// Belongs elsewhere
pub fn handle_damage(
    mut ev_damage: EventReader<DamageEvent>,
    mut units_query: Query<(&UnitId, &Children)>,
    mut health_indicator_query: Query<
        (&mut TextureAtlasSprite, &mut Visible),
        With<HealthIndicator>,
    >,
    mut commands: Commands,
) {
    for DamageEvent { entity, new_hp } in ev_damage.iter() {
        info!("Handling Damage Event");
        let (mut _unit, children) = units_query
            .get_mut(*entity)
            .expect("Could not find unit to damage");

        for &child in children.iter() {
            if let Ok((mut health_indicator, mut visible)) = health_indicator_query.get_mut(child) {
                info!("Updating health indicator");
                let ceil_health = new_hp.ceil().max(0.0) as u32;
                info!("new_hp: {:?}, ceil_health: {:?}", new_hp, ceil_health);
                if ceil_health == 0 {
                    commands.entity(*entity).despawn_recursive()
                } else if ceil_health < 10 {
                    visible.is_visible = true;
                    health_indicator.index = ceil_health - 1;
                }
            }
        }
    }
}
