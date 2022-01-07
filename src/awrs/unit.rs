use bevy::prelude::*;

use super::interface::{ActionResult, ActionResultEvent};

type UnitHealth = f32;

#[derive(Clone, PartialEq)]
pub struct Team(pub u32);

pub struct Selected;

pub struct UnitId(pub u32);

pub struct HealthIndicator;

pub fn _handle_attack_result(
    mut q_units: Query<(Entity, &UnitId)>,
    mut ev_attack_result: EventReader<ActionResultEvent>,
    mut ev_damage: EventWriter<DamageEvent>,
) {
    for ActionResultEvent(action_result) in ev_attack_result.iter() {
        if let ActionResult::AttackResult(damaged_units) = action_result {
            for (id, hp) in damaged_units {
                for (entity, unit_id) in q_units.iter_mut() {
                    if unit_id.0 == id.0 {
                        ev_damage.send(DamageEvent {
                            entity,
                            new_hp: *hp,
                        })
                    }
                }
            }
        }
    }
}

pub struct DamageEvent {
    entity: Entity,
    new_hp: UnitHealth,
}

pub fn handle_damage(
    mut ev_damage: EventReader<DamageEvent>,
    mut units_query: Query<(&UnitId, &Children)>,
    mut health_indicator_query: Query<&mut TextureAtlasSprite, With<HealthIndicator>>,
    mut commands: Commands,
) {
    for DamageEvent { entity, new_hp } in ev_damage.iter() {
        let (mut _unit, children) = units_query
            .get_mut(*entity)
            .expect("Could not find unit to damage");

        for &child in children.iter() {
            if let Ok(mut health_indicator) = health_indicator_query.get_mut(child) {
                let floored_health = new_hp.floor().max(0.0);
                health_indicator.index = floored_health as u32;
            }
        }

        if *new_hp <= 0.0 {
            commands.entity(*entity).despawn_recursive()
        }
    }
}
