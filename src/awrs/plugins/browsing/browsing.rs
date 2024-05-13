use bevy::ecs::event::*;
use bevy::prelude::*;

use crate::awrs::{
    engine::ScenarioState,
    register_inputs::InputEvent,
    resources::{
        cursor::{ChangeCursorEvent, CursorStyle, SelectEvent},
        map::ActiveTeam,
        state::{GameState, MenuState},
        unit::{Selected, UnitId},
    },
};

pub fn browse_select(
    mut ev_select: EventReader<SelectEvent>,
    mut commands: Commands,
    q_unit: Query<&UnitId>,
    mut next_game_state: ResMut<NextState<GameState>>,
    mut next_menu_state: ResMut<NextState<MenuState>>,
    active_team: Res<ActiveTeam>,
    scenario_state: Res<ScenarioState>,
) {
    for (select_event, id) in ev_select.read_with_id() {
        info!("Executing browse_select");
        warn!("Handling select event ({:?})", id);
        match select_event {
            SelectEvent::Entity(entity) => {
                if let Ok(UnitId(unit_id)) = q_unit.get(*entity) {
                    if scenario_state.unit_cannot_act(unit_id) {
                        info!("Unit cannot act.");
                        continue;
                    }

                    let unit = scenario_state
                        .get_unit(*unit_id)
                        .expect("Could not find unit in ScenarioState");

                    // Cannot select enemy units
                    let is_enemy = unit.team != active_team.team;
                    info!("active_team: {:?}", active_team);
                    if is_enemy {
                        info!("Can't select enemy units");
                        continue;
                    }

                    next_game_state.set(GameState::UnitMenu);
                    info!("Setting game state to UnitMenu");
                    commands.entity(*entity).insert(Selected);

                    // Potential alternatives to this:
                    // A resource that stores an optional handle to a unit (therefore can force only one unit selected at a time)
                    // A field on the Unit struct that says whether or not the unit is selected. (Doesn't feel very ECS?)
                }
            }
            SelectEvent::Tile(_tile) => {
                // Fire open menu event
                next_game_state.set(GameState::GameMenu);
                next_menu_state.set(MenuState::Open);
            }
        }
    }
}

pub fn listen_for_open_menu(
    mut ev_game_menu: ResMut<Events<InputEvent>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let mut reader = ev_game_menu.get_reader();
    let mut should_clear = false;
    for ev in reader.read(&ev_game_menu) {
        if matches!(ev, InputEvent::ToggleMenu) {
            next_state.set(GameState::GameMenu);
            should_clear = true;
        }
    }
    if should_clear {
        ev_game_menu.clear();
    }
}

pub fn open_browse(
    mut commands: Commands,
    mut ev_change_cursor: EventWriter<ChangeCursorEvent>,
    mut q_selected: Query<Entity, With<Selected>>,
) {
    // Change to Browse Cursor
    ev_change_cursor.send(ChangeCursorEvent(CursorStyle::Browse));

    // Deselect all units
    for selected_unit_entity in q_selected.iter_mut() {
        commands.entity(selected_unit_entity).remove::<Selected>();
    }
}
