use bevy::prelude::*;

use crate::awrs::engine::ScenarioState;
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
    mut st_game: ResMut<State<GameState>>,
    mut attacking_unit_query: Query<Entity, (With<Selected>, With<UnitId>)>,
    mut units_query: Query<(Entity, &UnitId), Without<Selected>>,
    mut commands: Commands,
    active_team: Res<ActiveTeam>,
    scenario_state: Res<ScenarioState>,
    mut ev_action: EventWriter<ActionEvent>,
    mut ev_select: EventReader<SelectEvent>,
) {
    for SelectEvent(defender_entity) in ev_select.iter() {
        info!("Executing target_select");
        // ! What happens if SelectEvent is triggered for a Selected unit?
        let (defender_entity, def_unit_id) = units_query
            .get_mut(*defender_entity)
            .expect("Unit doesn't exist?!");

        let def_unit = scenario_state
            .get_unit(def_unit_id.0)
            .expect("Couldn't find defending unit.");

        let is_enemy = def_unit.team != active_team.team;
        if !is_enemy {
            continue;
        }

        let attacker_entity = attacking_unit_query.single_mut();

        info!("Sending Attack Action Event");
        ev_action.send(ActionEvent(Action::Attack(
            attacker_entity,
            defender_entity,
        )));

        info!("Clearing selected unit");
        commands.entity(attacker_entity).remove::<Selected>();

        st_game
            .set(GameState::Browsing)
            .expect("Problem changing state");
    }
}
