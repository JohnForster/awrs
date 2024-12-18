use bevy::ecs::event;
use bevy::prelude::*;

use crate::awrs::engine::weapon::Delivery;
use crate::awrs::engine::{ScenarioState, Unit};
use crate::awrs::resources::action_event::Attack;
use crate::awrs::resources::tile::Tile;
use crate::awrs::resources::{
    action_event::{Action, ActionEvent},
    cursor::{ChangeCursorEvent, CursorStyle, SelectEvent},
    map::ActiveTeam,
    state::GameState,
    unit::{Selected, UnitId},
};

pub fn open_target_selection(
    mut ev_change_cursor: EventWriter<ChangeCursorEvent>,
    attacking_unit_query: Query<&UnitId, With<Selected>>,
    scenario_state: Res<ScenarioState>,
) {
    let attacker_id = attacking_unit_query.single();
    let attacker_unit = get_unit(&scenario_state, attacker_id);
    let weapon = attacker_unit.unit_type.value().weapon_one.unwrap();
    info!("Changed to Target Selection");
    if let Delivery::Splash(_) = weapon.delivery {
        ev_change_cursor.send(ChangeCursorEvent(CursorStyle::TargetSplash));
    } else {
        ev_change_cursor.send(ChangeCursorEvent(CursorStyle::Target));
    }
}

pub fn target_select(
    mut next_state: ResMut<NextState<GameState>>,
    mut attacking_unit_query: Query<(Entity, &UnitId), With<Selected>>,
    mut units_query: Query<(Entity, &UnitId), Without<Selected>>,
    mut commands: Commands,
    _active_team: Res<ActiveTeam>,
    scenario_state: Res<ScenarioState>,
    mut ev_action: EventWriter<ActionEvent>,
    mut ev_select: EventReader<SelectEvent>,
) {
    let (attacker_entity, attacker_id) = attacking_unit_query.single_mut();
    for (select_event, id) in ev_select.read_with_id() {
        warn!("Handling event ({:?})", id);
        match select_event {
            SelectEvent::Entity { entity, context } => {
                if context.game_state != GameState::ChooseTarget {
                    continue;
                }
                info!("Executing target_select");
                // ! What happens if SelectEvent is triggered for a Selected unit?
                let defender = units_query.get_mut(*entity);
                match defender {
                    Ok((defender_entity, _def_unit_id)) => {
                        info!("Sending Attack Unit Action Event");
                        ev_action.send(ActionEvent(Action::Attack(Attack::Unit(
                            attacker_entity,
                            defender_entity,
                        ))));
                        info!("Clearing selected unit");
                    }
                    Err(_) => {
                        warn!("Select event was fired for entity, but entity was not found.");
                        warn!("Attacking self");
                        let attacker_unit = get_unit(&scenario_state, attacker_id);
                        send_attack_ground_event(
                            &attacker_entity,
                            &Tile::from(attacker_unit.position),
                            &mut ev_action,
                        )
                    }
                }
            }
            SelectEvent::Tile { tile, context } => {
                if context.game_state != GameState::ChooseTarget {
                    continue;
                }
                send_attack_ground_event(&attacker_entity, tile, &mut ev_action);
            }
        };
        clear_selected_and_reset(&mut commands, attacker_entity, &mut next_state);
        // Removed because this exits the targeting state before the result comes back.
        // Need to decide what happens in between.
    }
    ev_select.clear();
}

fn clear_selected_and_reset(
    commands: &mut Commands,
    attacker_entity: Entity,
    next_state: &mut ResMut<NextState<GameState>>,
) {
    commands.entity(attacker_entity).remove::<Selected>();

    warn!("Switching back to browsing");
    next_state.set(GameState::Browsing);
}

fn send_attack_ground_event(
    attacker_entity: &Entity,
    tile: &crate::awrs::resources::tile::Tile,
    ev_action: &mut EventWriter<ActionEvent>,
) {
    let action = Action::Attack(Attack::Ground(attacker_entity.clone(), *tile));
    info!("Sending Attack Ground Action Event");
    ev_action.send(ActionEvent(action));
}

fn get_unit<'a>(scenario_state: &'a Res<ScenarioState>, unit_id: &UnitId) -> &'a Unit {
    scenario_state
        .get_unit(unit_id.0)
        .expect("Couldn't find unit.")
}
