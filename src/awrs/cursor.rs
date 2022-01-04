use bevy::prelude::*;

use crate::awrs::game::GameState;

use super::constants::*;
use super::engine::{Contains, ScenarioState};
use super::game::AppState;
use super::map::ActiveTeam;
use super::register_inputs::InputEvent;
use super::sprite_loading::CursorAtlas;
use super::tile::Tile;
use super::unit::*;

pub struct Cursor;

pub fn open_browse(mut ev_change_cursor: EventWriter<ChangeCursorEvent>) {
    ev_change_cursor.send(ChangeCursorEvent(CursorStyle::Browse));
}

pub fn create_cursor(mut commands: Commands, ui_atlas: Res<CursorAtlas>) {
    info!("Creating Cursor");
    let tile = Tile { x: 0, y: 0 };
    let starting_position = Vec3::new(tile.x as f32, tile.y as f32, 0.0) * TILE_SIZE;
    let adjustment = Vec3::new(4.0, -5.0, 2.0);

    // Combine these into the Cursor struct?
    commands
        .spawn()
        .insert(Cursor)
        .insert(GlobalTransform::default())
        .insert(Transform::from_translation(starting_position))
        .with_children(|parent| {
            parent
                .spawn()
                .insert(GlobalTransform::default())
                .insert_bundle(SpriteSheetBundle {
                    texture_atlas: ui_atlas.atlas_handle.clone(),
                    sprite: TextureAtlasSprite::new(0),
                    transform: Transform::from_translation(adjustment),
                    ..Default::default()
                });
        });
}

pub enum CursorStyle {
    Browse,
    Target,
    None,
}

pub struct ChangeCursorEvent(pub CursorStyle);

pub fn handle_change_cursor(
    mut ev_change_cursor: EventReader<ChangeCursorEvent>,
    mut q_cursor_children: Query<&mut Children, With<Cursor>>,
    mut q_sprite: Query<&mut TextureAtlasSprite>,
) {
    for ChangeCursorEvent(cursor_style) in ev_change_cursor.iter() {
        let sprite_index = match cursor_style {
            CursorStyle::Browse => 0,
            CursorStyle::Target => 1,
            CursorStyle::None => 9,
        };
        info!("Changing cursor sprite index to {:?}", sprite_index);
        let cursor_children = q_cursor_children.single_mut().expect("No Cursor Found?!");

        for child in cursor_children.iter() {
            if let Ok(mut cursor_sprite) = q_sprite.get_mut(*child) {
                cursor_sprite.index = sprite_index;
            }
        }
    }
}

pub fn handle_cursor_move(
    mut ev_input_event: EventReader<InputEvent>,
    mut q_cursor: Query<&mut Transform, With<Cursor>>,
    scenario_state: Res<ScenarioState>,
) {
    for input_event in ev_input_event.iter() {
        let mut transform = q_cursor.single_mut().expect("Should be one cursor.");

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
    mut q_cursor: Query<&Transform, With<Cursor>>,
    q_units: Query<(Entity, &Transform), With<UnitId>>,
    mut ev_select: EventWriter<SelectEvent>,
) {
    for input_event in ev_input_event.iter() {
        let mut transform = q_cursor.single_mut().expect("Should be one cursor.");

        let unit = match input_event {
            &InputEvent::Select => {
                let cursor_transform = q_cursor.single().expect("No Cursor found?!");
                let cursor_tile = Tile::from(*cursor_transform);

                let maybe_unit = q_units
                    .iter()
                    .find(|(_, transform)| Tile::from(**transform) == cursor_tile);

                if let Some(tuple) = maybe_unit {
                    let entity = tuple.0;

                    ev_select.send(SelectEvent(entity));
                }
            }

            _ => break, // Could add select here?
        };
    }
}

// ? Do we need this select event, or could this be bundled into handle_cursor_select?
pub struct SelectEvent(pub Entity);

pub fn browse_select(
    mut ev_select: EventReader<SelectEvent>,
    mut commands: Commands,
    q_unit: Query<&UnitId>,
    mut game_state: ResMut<State<AppState>>,
    active_team: Res<ActiveTeam>,
    scenario_state: Res<ScenarioState>,
) {
    for SelectEvent(entity) in ev_select.iter() {
        let UnitId(unit_id) = q_unit.get(*entity).expect("Unit doesn't exist?!");

        let unit = scenario_state
            .get_unit(*unit_id)
            .expect("Could not find unit in ScenarioState");

        // Cannot select enemy units
        let is_enemy = unit.team != active_team.team.0;
        if is_enemy {
            continue;
        }

        // Potential alternatives to this:
        // A resource that stores an optional handle to a unit (therefore can force only one unit selected at a time)
        // A field on the Unit struct that says whether or not the unit is selected. (Doesn't feel very ECS?)
        commands.entity(*entity).insert(Selected);

        info!("Setting game state to UnitMenu");
        game_state
            .set(AppState::InGame(GameState::UnitMenu))
            .expect("Problem changing state");
    }
}
