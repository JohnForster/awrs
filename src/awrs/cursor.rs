use bevy::prelude::*;

use crate::awrs::game::GameState;

use super::constants::*;
use super::game::AppState;
use super::map::ActiveTeam;
use super::sprite_loading::CursorAtlas;
use super::tile::Tile;
use super::unit::*;

pub struct Cursor;

pub fn open_browse(mut ev_change_cursor: EventWriter<ChangeCursorEvent>) {
    ev_change_cursor.send(ChangeCursorEvent(CursorStyle::Browse));
}

pub fn create_cursor(mut commands: Commands, ui_atlas: Res<CursorAtlas>) {
    info!("Creating Cursor");
    let x = 0;
    let y = 0;
    let starting_position = Vec3::new(x as f32, y as f32, 0.0) * TILE_SIZE;
    let adjustment = Vec3::new(4.0, -5.0, 2.0);

    // Combine these into the Cursor struct?
    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: ui_atlas.atlas_handle.clone(),
            sprite: TextureAtlasSprite::new(0),
            transform: Transform::from_translation(starting_position + adjustment),
            ..Default::default()
        })
        .insert(Cursor)
        .insert(Moveable)
        .insert(Timer::from_seconds(0.075, false));
}

pub enum CursorStyle {
    Browse,
    Target,
    None,
}

pub struct ChangeCursorEvent(pub CursorStyle);

pub fn handle_change_cursor(
    mut ev_change_cursor: EventReader<ChangeCursorEvent>,
    mut q_cursor_sprite: Query<&mut TextureAtlasSprite, With<Cursor>>,
) {
    for ChangeCursorEvent(cursor_style) in ev_change_cursor.iter() {
        let sprite_index = match cursor_style {
            CursorStyle::Browse => 0,
            CursorStyle::Target => 1,
            CursorStyle::None => 9,
        };
        info!("Changing cursor sprite index to {:?}", sprite_index);
        let mut cursor_sprite = q_cursor_sprite.single_mut().expect("No Cursor Found?!");
        cursor_sprite.index = sprite_index;
    }
}

pub struct SelectEvent(pub Entity);

pub fn select_unit(
    keyboard_input: Res<Input<KeyCode>>,
    q_cursor: Query<&Transform, With<Cursor>>,
    q_units: Query<(Entity, &Transform), With<Unit>>,
    mut ev_select: EventWriter<SelectEvent>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
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
}

pub fn browse_select(
    mut ev_select: EventReader<SelectEvent>,
    mut commands: Commands,
    q_unit: Query<&Unit>,
    mut r_game_state: ResMut<State<AppState>>,
    r_active_team: Res<ActiveTeam>,
) {
    for SelectEvent(entity) in ev_select.iter() {
        let unit = q_unit.get(*entity).expect("Unit doesn't exist?!");

        // Cannot select enemy units
        let is_enemy = unit.team != r_active_team.team;
        if is_enemy {
            continue;
        }

        // Potential alternatives to this:
        // A resource that stores an optional handle to a unit (therefore can force only one unit selected at a time)
        // A field on the Unit struct that says whether or not the unit is selected. (Doesn't feel very ECS?)
        commands.entity(*entity).insert(Selected);

        info!("Setting game state to UnitMenu");
        r_game_state
            .set(AppState::InGame(GameState::UnitMenu))
            .expect("Problem changing state");
    }
}
