use advance_craft_engine::Contains;
use bevy::prelude::*;

use crate::awrs::{
    constants::TILE_SIZE,
    register_inputs::InputEvent,
    resources::{
        cursor::{EventContext, SelectEvent},
        scenario::ScenarioState,
        state::GameState,
        tile::Tile,
        unit::UnitId,
    },
};

pub struct CursorPlugin;

fn cursor_active(game_state: Res<State<GameState>>) -> bool {
    match game_state.get() {
        GameState::Browsing | GameState::ChooseTarget => true,
        _ => false,
    }
}

impl Plugin for CursorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (handle_cursor_move, handle_cursor_select).run_if(cursor_active),
        );
    }
}

#[derive(Component)]
pub struct Cursor;

pub fn handle_cursor_move(
    mut ev_input_event: EventReader<InputEvent>,
    mut q_cursor: Query<&mut Transform, With<Cursor>>,
    scenario_state: Res<ScenarioState>,
) {
    for input_event in ev_input_event.read() {
        let mut transform = q_cursor.single_mut();

        let (dx, dy): (i32, i32) = match input_event {
            &InputEvent::Up => (0, 1),
            &InputEvent::Down => (0, -1),
            &InputEvent::Left => (-1, 0),
            &InputEvent::Right => (1, 0),
            _ => break, // Could add select here?
        };

        let new_x = (transform.translation.x / TILE_SIZE + dx as f32).round();
        let new_y = (transform.translation.y / TILE_SIZE + dy as f32).round();

        if scenario_state.contains(&new_x, &new_y) {
            transform.translation.x = new_x * TILE_SIZE;
            transform.translation.y = new_y * TILE_SIZE;
        }
    }
}

pub fn handle_cursor_select(
    mut ev_input_event: EventReader<InputEvent>,
    q_cursor: Query<&Transform, With<Cursor>>,
    q_units: Query<(Entity, &Transform), With<UnitId>>,
    game_state: Res<State<GameState>>,
    mut ev_select: EventWriter<SelectEvent>,
) {
    for input_event in ev_input_event.read() {
        match input_event {
            &InputEvent::Select => {
                let cursor_transform = q_cursor.single();
                let cursor_tile = Tile::from(*cursor_transform);

                let maybe_unit = q_units
                    .iter()
                    .find(|(_, transform)| Tile::from(**transform) == cursor_tile);

                let context = EventContext {
                    game_state: *game_state.get(),
                };
                match maybe_unit {
                    Some(tuple) => {
                        let entity = tuple.0;
                        ev_select.send(SelectEvent::Entity { entity, context })
                    }
                    None => ev_select.send(SelectEvent::Tile {
                        tile: cursor_tile,
                        context,
                    }),
                }
            }

            _ => break, // Could add select here?
        };
    }
}
