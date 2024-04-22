use bevy::prelude::*;

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

pub fn open_target_selection(mut ev_change_cursor: EventWriter<ChangeCursorEvent>) {
    info!("Changed to Target Selection");
    ev_change_cursor.send(ChangeCursorEvent(CursorStyle::Target));
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
    for select_event in ev_select.read() {
        let (attacker_entity, attacker_id) = attacking_unit_query.single_mut();
        match select_event {
            SelectEvent::Entity(defender_entity) => {
                info!("Executing target_select");
                // ! What happens if SelectEvent is triggered for a Selected unit?
                let defender = units_query.get_mut(*defender_entity);
                match defender {
                    Result::Ok((defender_entity, _def_unit_id)) => {
                        info!("Sending Attack Unit Action Event");
                        ev_action.send(ActionEvent(Action::Attack(Attack::Unit(
                            attacker_entity,
                            defender_entity,
                        ))));
                        info!("Clearing selected unit");
                    }
                    Result::Err(_) => {
                        let attacker_unit = get_unit(&scenario_state, attacker_id);
                        send_attack_ground_event(
                            &mut attacking_unit_query,
                            &Tile::from(attacker_unit.position),
                            &mut ev_action,
                        )
                    }
                }
            }
            SelectEvent::Tile(tile) => {
                send_attack_ground_event(&mut attacking_unit_query, tile, &mut ev_action);
            }
        }
        clear_selected_and_reset(&mut commands, attacker_entity, &mut next_state);
    }
}

fn clear_selected_and_reset(
    commands: &mut Commands,
    attacker_entity: Entity,
    next_state: &mut ResMut<NextState<GameState>>,
) {
    commands.entity(attacker_entity).remove::<Selected>();
    next_state.set(GameState::Browsing);
}

fn send_attack_ground_event(
    attacking_unit_query: &mut Query<(Entity, &UnitId), With<Selected>>,
    tile: &crate::awrs::resources::tile::Tile,
    ev_action: &mut EventWriter<ActionEvent>,
) {
    let attacker_entity = attacking_unit_query.single_mut().0;
    let action = Action::Attack(Attack::Ground(attacker_entity, *tile));
    info!("Sending Attack Ground Action Event");
    ev_action.send(ActionEvent(action));
}

fn get_unit<'a>(scenario_state: &'a Res<ScenarioState>, unit_id: &UnitId) -> &'a Unit {
    scenario_state
        .get_unit(unit_id.0)
        .expect("Couldn't find unit.")
}
