use bevy::{app::Events, prelude::*};

use crate::awrs::{
    engine::ScenarioState,
    register_inputs::InputEvent,
    resources::{
        cursor::{ChangeCursorEvent, CursorStyle, SelectEvent},
        map::ActiveTeam,
        state::GameState,
        unit::{Selected, UnitId},
    },
};

pub fn browse_select(
    mut ev_select: EventReader<SelectEvent>,
    mut commands: Commands,
    q_unit: Query<&UnitId>,
    mut game_state: ResMut<State<GameState>>,
    active_team: Res<ActiveTeam>,
    scenario_state: Res<ScenarioState>,
) {
    for SelectEvent(entity) in ev_select.iter() {
        info!("Executing browse_select");
        if let Ok(UnitId(unit_id)) = q_unit.get(*entity) {
            let unit = scenario_state
                .get_unit(*unit_id)
                .expect("Could not find unit in ScenarioState");

            // Cannot select enemy units
            let is_enemy = unit.team != active_team.team;
            if is_enemy {
                continue;
            }

            // Potential alternatives to this:
            // A resource that stores an optional handle to a unit (therefore can force only one unit selected at a time)
            // A field on the Unit struct that says whether or not the unit is selected. (Doesn't feel very ECS?)
            commands.entity(*entity).insert(Selected);

            info!("Setting game state to UnitMenu");
            game_state
                .set(GameState::UnitMenu)
                .expect("Problem changing state");
        }
    }
}

pub fn listen_for_open_menu(
    mut ev_game_menu: ResMut<Events<InputEvent>>,
    mut game_state: ResMut<State<GameState>>,
) {
    let mut reader = ev_game_menu.get_reader();
    let mut should_clear = false;
    for ev in reader.iter(&ev_game_menu) {
        if matches!(ev, InputEvent::ToggleMenu) {
            game_state
                .push(GameState::GameMenu)
                .expect("Error changing state");
            should_clear = true;
        }
    }
    if should_clear {
        ev_game_menu.clear();
    }
}

pub fn open_browse(mut ev_change_cursor: EventWriter<ChangeCursorEvent>) {
    ev_change_cursor.send(ChangeCursorEvent(CursorStyle::Browse));
}
