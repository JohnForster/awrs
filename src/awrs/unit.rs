use bevy::prelude::*;

use super::{
    cell::Cell,
    constants::TILE_SIZE,
    game::{AppState, GameState},
    map::GameMap,
};

#[derive(Clone, Copy)]
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

#[derive(Clone, PartialEq, Copy)]
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

#[derive(Clone, Copy)]
pub struct Unit {
    pub unit_type: usize,
    pub team: Team,
    pub location: Cell,
    pub health: UnitHealth,
    // pub ammo: Ammo,
    // etc. etc..
}

#[derive(Clone, Copy)]
pub struct UnitId(pub usize);

#[derive(Bundle)]
pub struct UnitBundle {
    pub id: UnitId,
    pub data: Unit,
    #[bundle]
    pub sprite: SpriteSheetBundle,
}

pub struct HealthIndicator;

// Very similar to moving cursor.
// Could have Movable struct component so that this can be reused?
// Or could extract movement logic into a separate function?
pub fn handle_unit_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut unit_query: Query<(&mut Transform, &mut Unit), With<Selected>>,
    game_map_query: Query<&GameMap>,
    mut game_state: ResMut<State<AppState>>,
) {
    let game_map = game_map_query
        .single()
        .expect("Trying to move a unit when there is no map?!");

    for (mut transform, mut unit) in unit_query.iter_mut() {
        if keyboard_input.just_pressed(KeyCode::W) && unit.location.y < game_map.height {
            transform.translation.y += 1.0 * TILE_SIZE;
            unit.location.y += 1;
        }

        if keyboard_input.just_pressed(KeyCode::A) && unit.location.x > 0 {
            transform.translation.x -= 1.0 * TILE_SIZE;
            unit.location.x -= 1;
        }

        if keyboard_input.just_pressed(KeyCode::S) && unit.location.y > 0 {
            transform.translation.y -= 1.0 * TILE_SIZE;
            unit.location.y -= 1;
        }

        if keyboard_input.just_pressed(KeyCode::D) && unit.location.x < game_map.width {
            transform.translation.x += 1.0 * TILE_SIZE;
            unit.location.x += 1;
        }

        if keyboard_input.just_pressed(KeyCode::Space) {
            info!("Returning to UnitMenu state");
            game_state
                .set(AppState::InGame(GameState::UnitMenu))
                .expect("Problem changing state");
        }
    }
}

pub fn calculate_damage(_attacking_unit: &Unit, _defending_unit: &Unit) -> (f32, f32) {
    return (2.0, 4.0);
}

pub struct AttackEvent(pub Entity, pub Entity);

pub fn handle_attack(
    units_query: Query<&Unit>,
    mut ev_attack: EventReader<AttackEvent>,
    mut ev_damage: EventWriter<DamageEvent>,
) {
    for ev in ev_attack.iter() {
        info!("Attacking!");
        let attacker = units_query.get(ev.0).expect("Could not find attacker");
        let defender = units_query.get(ev.1).expect("Could not find defender");

        let (att_damage, def_damage) = calculate_damage(attacker, defender);

        ev_damage.send(DamageEvent(ev.0, att_damage));
        ev_damage.send(DamageEvent(ev.1, def_damage));
    }
}

pub struct DamageEvent(pub Entity, pub f32);

pub fn handle_damage(
    mut ev_damage: EventReader<DamageEvent>,
    mut units_query: Query<(&mut Unit, &Children)>,
    mut health_indicator_query: Query<&mut TextureAtlasSprite, With<HealthIndicator>>,
    mut commands: Commands,
) {
    for DamageEvent(entity, damage) in ev_damage.iter() {
        let (mut unit, children) = units_query
            .get_mut(*entity)
            .expect("Could not find unit to damage");

        unit.health.0 -= damage;

        // Maybe updating health indicator should be moved to a UI system?
        for &child in children.iter() {
            if let Ok(mut health_indicator) = health_indicator_query.get_mut(child) {
                let floored_health = unit.health.0.floor().max(0.0);
                health_indicator.index = floored_health as u32;
            }
        }

        if unit.health.0 <= 0.0 {
            commands.entity(*entity).despawn_recursive()
        }
    }
}
