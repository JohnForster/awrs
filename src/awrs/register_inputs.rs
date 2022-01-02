use bevy::prelude::*;

use super::tile::Tile;

pub enum InputEvent {
    Left,
    Right,
    Up,
    Down,
    Select,
    Tile(Tile),
    // Remove these once proper UI is built
    Move,
    Attack,
    Cancel,
}

pub fn register_inputs(keyboard_input: Res<Input<KeyCode>>, mut ev_move: EventWriter<InputEvent>) {
    if keyboard_input.just_pressed(KeyCode::W) {
        return ev_move.send(InputEvent::Up);
    }

    if keyboard_input.just_pressed(KeyCode::A) {
        return ev_move.send(InputEvent::Up);
    }

    if keyboard_input.just_pressed(KeyCode::S) {
        return ev_move.send(InputEvent::Up);
    }

    if keyboard_input.just_pressed(KeyCode::D) {
        return ev_move.send(InputEvent::Up);
    }

    if keyboard_input.just_pressed(KeyCode::Space) {
        return ev_move.send(InputEvent::Select);
    }

    // Remove these once proper UI is built
    if keyboard_input.just_pressed(KeyCode::M) {
        return ev_move.send(InputEvent::Move);
    }

    if keyboard_input.just_pressed(KeyCode::T) {
        return ev_move.send(InputEvent::Attack);
    }

    if keyboard_input.just_pressed(KeyCode::C) {
        return ev_move.send(InputEvent::Cancel);
    }
}

// ------------------------------- OLD -------------------------------
// pub fn register_inputs(keyboard_input: Res<Input<KeyCode>>, mut ev_move: EventWriter<InputEvent>) {
//     let mut cell_change = CellChange { x: 0, y: 0 };
//     if keyboard_input.just_pressed(KeyCode::W) {
//         cell_change.y += 1;
//     }

//     if keyboard_input.just_pressed(KeyCode::A) {
//         cell_change.x -= 1;
//     }

//     if keyboard_input.just_pressed(KeyCode::S) {
//         cell_change.y -= 1;
//     }

//     if keyboard_input.just_pressed(KeyCode::D) {
//         cell_change.x += 1;
//     }

//     if !(cell_change.x == 0 && cell_change.y == 0) {
//         info!("Sending input event! {:?}", cell_change);
//         ev_move.send(InputEvent(cell_change))
//     }
// }
