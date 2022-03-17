use bevy::prelude::*;

use super::resources::tile::Tile;

#[derive(Debug)]
pub enum InputEvent {
    Left,
    Right,
    Up,
    Down,
    Select,
    ToggleMenu,
    _Tile(Tile), // For eventual touch/mouse input.
    // Remove these once proper UI is built
    Move,
    Attack,
    Cancel,
    EndTurn,
}

pub fn register_inputs(keyboard_input: Res<Input<KeyCode>>, mut ev_input: EventWriter<InputEvent>) {
    if keyboard_input.just_pressed(KeyCode::W) {
        return ev_input.send(InputEvent::Up);
    }

    if keyboard_input.just_pressed(KeyCode::A) {
        return ev_input.send(InputEvent::Left);
    }

    if keyboard_input.just_pressed(KeyCode::S) {
        return ev_input.send(InputEvent::Down);
    }

    if keyboard_input.just_pressed(KeyCode::D) {
        return ev_input.send(InputEvent::Right);
    }

    if keyboard_input.just_pressed(KeyCode::Space) {
        return ev_input.send(InputEvent::Select);
    }

    if keyboard_input.just_pressed(KeyCode::Return) {
        return ev_input.send(InputEvent::ToggleMenu);
    }

    // Remove these once proper UI is built
    if keyboard_input.just_pressed(KeyCode::M) {
        return ev_input.send(InputEvent::Move);
    }

    if keyboard_input.just_pressed(KeyCode::T) {
        return ev_input.send(InputEvent::Attack);
    }

    if keyboard_input.just_pressed(KeyCode::C) {
        return ev_input.send(InputEvent::Cancel);
    }

    if keyboard_input.just_pressed(KeyCode::E) {
        return ev_input.send(InputEvent::EndTurn);
    }
}
