use bevy::prelude::*;

use super::{
    cursor::{ChangeCursorEvent, CursorStyle, SelectEvent},
    engine::ScenarioState,
    game::{AppState, GameState},
    interface::{Action, ActionEvent},
    map::ActiveTeam,
    unit::{Selected, UnitId},
};

pub fn open_target_selection(mut ev_change_cursor: EventWriter<ChangeCursorEvent>) {
    info!("Changed to Target Selection");
    ev_change_cursor.send(ChangeCursorEvent(CursorStyle::Target));
}

pub fn target_select(
    mut game_state: ResMut<State<AppState>>,
    mut attacking_unit_query: Query<Entity, (With<Selected>, With<UnitId>)>,
    mut units_query: Query<(Entity, &UnitId), Without<Selected>>,
    mut commands: Commands,
    active_team: Res<ActiveTeam>,
    scenario_state: Res<ScenarioState>,
    mut ev_action: EventWriter<ActionEvent>,
    mut ev_select: EventReader<SelectEvent>,
) {
    for SelectEvent(defender_entity) in ev_select.iter() {
        // ! What happens if SelectEvent is triggered for a Selected unit?
        let (defender_entity, def_unit_id) = units_query
            .get_mut(*defender_entity)
            .expect("Unit doesn't exist?!");

        let def_unit = scenario_state
            .get_unit(def_unit_id.0)
            .expect("Couldn't find defending unit.");

        let is_enemy = def_unit.team != active_team.team.0;
        if !is_enemy {
            continue;
        }

        let attacker_entity = attacking_unit_query
            .single_mut()
            .expect("Trying to attack a target without a unit selected!");

        ev_action.send(ActionEvent(Action::Attack(
            attacker_entity,
            defender_entity,
        )));

        commands.entity(attacker_entity).remove::<Selected>();

        game_state
            .set(AppState::InGame(GameState::Browsing))
            .expect("Problem changing state");
    }
}
