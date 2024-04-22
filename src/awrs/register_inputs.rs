use bevy::prelude::*;

use super::resources::tile::Tile;

#[derive(Debug, Event)]
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

pub fn register_inputs(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut ev_input: EventWriter<InputEvent>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyW) {
        ev_input.send(InputEvent::Up);
        return;
    }

    if keyboard_input.just_pressed(KeyCode::KeyA) {
        ev_input.send(InputEvent::Left);
        return;
    }

    if keyboard_input.just_pressed(KeyCode::KeyS) {
        ev_input.send(InputEvent::Down);
        return;
    }

    if keyboard_input.just_pressed(KeyCode::KeyD) {
        ev_input.send(InputEvent::Right);
        return;
    }

    if keyboard_input.just_pressed(KeyCode::Space) {
        ev_input.send(InputEvent::Select);
        return;
    }

    if keyboard_input.just_pressed(KeyCode::Enter) {
        ev_input.send(InputEvent::ToggleMenu);
        return;
    }

    // Remove these once proper UI is built
    if keyboard_input.just_pressed(KeyCode::KeyM) {
        ev_input.send(InputEvent::Move);
        return;
    }

    if keyboard_input.just_pressed(KeyCode::KeyT) {
        ev_input.send(InputEvent::Attack);
        return;
    }

    if keyboard_input.just_pressed(KeyCode::KeyC) {
        ev_input.send(InputEvent::Cancel);
        return;
    }

    if keyboard_input.just_pressed(KeyCode::KeyE) {
        ev_input.send(InputEvent::EndTurn);
        return;
    }
}
