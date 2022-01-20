use bevy::prelude::*;

use super::resources::tile::Tile;

#[derive(Debug)]
pub enum InputEvent {
    Left,
    Right,
    Up,
    Down,
    Select,
    _Tile(Tile), // For eventual touch/mouse input.
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
        return ev_move.send(InputEvent::Left);
    }

    if keyboard_input.just_pressed(KeyCode::S) {
        return ev_move.send(InputEvent::Down);
    }

    if keyboard_input.just_pressed(KeyCode::D) {
        return ev_move.send(InputEvent::Right);
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
