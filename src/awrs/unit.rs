use bevy::prelude::*;

use super::{
    engine::ScenarioState,
    interface::{ActionResult, ActionResultEvent},
    map::GameMap,
    plugins::UnitMove,
    register_inputs::InputEvent,
    sprite_loading::UIAtlas,
    tile::{Tile, TILE_SIZE},
};

type UnitHealth = f32;

#[derive(Clone, PartialEq)]
pub struct Team(pub u32);

pub struct Selected;

pub struct UnitId(pub u32);

pub struct HealthIndicator;

pub fn handle_attack_result(
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
        let (mut unit, children) = units_query
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

pub struct AddUnitMoveStepEvent(pub Tile);

pub struct MoveStep;

pub fn add_move_step(
    mut ev_add_move: EventReader<AddUnitMoveStepEvent>,
    mut res_unit_move: ResMut<UnitMove>,
    res_ui_atlas: Res<UIAtlas>,
    mut commands: Commands,
) {
    for AddUnitMoveStepEvent(tile) in ev_add_move.iter() {
        let mut sprite = TextureAtlasSprite::new(0);
        let mut color = Color::WHITE;
        color.set_a(0.5);
        sprite.color = color;

        let entity = commands
            .spawn_bundle(SpriteSheetBundle {
                texture_atlas: res_ui_atlas.atlas_handle.clone(),
                sprite,
                transform: Transform::from_translation(Vec3::new(
                    tile.x as f32 * TILE_SIZE,
                    tile.y as f32 * TILE_SIZE,
                    5.0,
                )),
                ..Default::default()
            })
            .insert(MoveStep)
            .id();

        res_unit_move.tiles.push(*tile);
        res_unit_move.entities.push(entity);
    }
}

pub fn move_unit(
    mut q_selected_unit: Query<(&UnitId, &mut Transform), With<Selected>>,
    scenario_state: Res<ScenarioState>,
    mut ev_input: EventReader<InputEvent>,
    mut ev_move_step: EventWriter<AddUnitMoveStepEvent>,
) {
    for input_event in ev_input.iter() {
        let (UnitId(unit_id), mut transform) = q_selected_unit
            .single_mut()
            // Maybe allow this to fail gracefully, so that we don't error if there is Select -> Direction within same tick.
            .expect("Should be one selected unit");

        let (dx, dy): (i32, i32) = match input_event {
            &InputEvent::Up => (0, 1),
            &InputEvent::Down => (0, -1),
            &InputEvent::Left => (-1, 0),
            &InputEvent::Right => (1, 0),
            _ => break, // Could add select here?
        };

        let current_pos = Tile::from(*transform);

        let valid_tiles = scenario_state.get_moveable_tiles(*unit_id);

        let maybe_tile = valid_tiles.into_iter().find(|tile| {
            tile.x as i32 == current_pos.x as i32 + dx && tile.y as i32 == current_pos.y as i32 + dy
        });

        if let Some(tile) = maybe_tile {
            ev_move_step.send(AddUnitMoveStepEvent(Tile {
                x: tile.x,
                y: tile.y,
            }));
            transform.translation.x += dy as f32 * TILE_SIZE;
            transform.translation.y += dy as f32 * TILE_SIZE;
        }
    }
}
