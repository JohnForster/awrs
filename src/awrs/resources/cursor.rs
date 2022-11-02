use bevy::prelude::*;

use crate::awrs::engine::engine::Contains;

use crate::awrs::{engine::ScenarioState, register_inputs::InputEvent};

use super::{
    tile::{Tile, TILE_SIZE},
    unit::UnitId,
};

pub enum CursorStyle {
    Browse,
    Target,
    None,
}

pub struct ChangeCursorEvent(pub CursorStyle);

#[derive(Component)]
pub struct Cursor;

pub fn handle_cursor_move(
    mut ev_input_event: EventReader<InputEvent>,
    mut q_cursor: Query<&mut Transform, With<Cursor>>,
    scenario_state: Res<ScenarioState>,
) {
    for input_event in ev_input_event.iter() {
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

// ? Do we need this select event, or could this be bundled into handle_cursor_select?
pub enum SelectEvent {
    Entity(Entity),
    Tile(Tile),
}

pub fn handle_cursor_select(
    mut ev_input_event: EventReader<InputEvent>,
    q_cursor: Query<&Transform, With<Cursor>>,
    q_units: Query<(Entity, &Transform), With<UnitId>>,
    mut ev_select: EventWriter<SelectEvent>,
) {
    for input_event in ev_input_event.iter() {
        match input_event {
            &InputEvent::Select => {
                let cursor_transform = q_cursor.single();
                let cursor_tile = Tile::from(*cursor_transform);

                let maybe_unit = q_units
                    .iter()
                    .find(|(_, transform)| Tile::from(**transform) == cursor_tile);

                match maybe_unit {
                    Some(tuple) => {
                        let entity = tuple.0;

                        ev_select.send(SelectEvent::Entity(entity));
                    }
                    None => ev_select.send(SelectEvent::Tile(cursor_tile)),
                }
            }

            _ => break, // Could add select here?
        };
    }
}
