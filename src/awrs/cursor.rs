use bevy::prelude::*;

use crate::awrs::game::GameState;

use super::constants::*;
use super::engine::ScenarioState;
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
    let tile = Tile { x: 0, y: 0 };
    let starting_position = Vec3::new(tile.x as f32, tile.y as f32, 0.0) * TILE_SIZE;
    let adjustment = Vec3::new(4.0, -5.0, 2.0);

    // Combine these into the Cursor struct?
    commands
        .spawn()
        .insert(Cursor)
        .insert(Transform::from_translation(starting_position))
        .with_children(|parent| {
            parent.spawn_bundle(SpriteSheetBundle {
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
    q_units: Query<(Entity, &Transform), With<UnitId>>,
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
