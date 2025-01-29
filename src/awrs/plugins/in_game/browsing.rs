use bevy::ecs::event::*;
use bevy::prelude::*;

use crate::awrs::{
    register_inputs::InputEvent,
    resources::{
        cursor::{ChangeCursorEvent, CursorStyle, SelectEvent},
        map::ActiveTeam,
        scenario::ScenarioState,
        state::{GameState, MenuState},
        unit::{Selected, UnitId},
    },
};

pub struct BrowsingPlugin;

impl Plugin for BrowsingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Browsing), open_browse)
            .add_systems(
                Update,
                (browse_select, listen_for_open_menu).run_if(in_state(GameState::Browsing)),
            );
    }
}

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
            SelectEvent::Entity { entity, context } => {
                if context.game_state != GameState::Browsing {
                    continue;
                }
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
                    if is_enemy {
                        info!("Can't select enemy units");
                        continue;
                    }

                    next_game_state.set(GameState::UnitMenu);
                    commands.entity(*entity).insert(Selected);

                    // Potential alternatives to this:
                    // A resource that stores an optional handle to a unit (therefore can force only one unit selected at a time)
                    // A field on the Unit struct that says whether or not the unit is selected. (Doesn't feel very ECS?)
                }
            }
            SelectEvent::Tile { context, .. } => {
                if context.game_state != GameState::Browsing {
                    continue;
                }
                // Fire open menu event
                next_game_state.set(GameState::GameMenu);
                next_menu_state.set(MenuState::Open);
            }
        }
    }
}

pub fn listen_for_open_menu(
    mut ev_game_menu: ResMut<Events<InputEvent>>,
    mut next_menu_state: ResMut<NextState<MenuState>>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    let mut cursor = ev_game_menu.get_cursor();
    let mut should_clear = false;
    for ev in cursor.read(&ev_game_menu) {
        if matches!(ev, InputEvent::ToggleMenu) {
            next_game_state.set(GameState::GameMenu);
            next_menu_state.set(MenuState::Open);
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
