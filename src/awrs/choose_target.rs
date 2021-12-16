use bevy::prelude::*;

use super::{
    cell::Cell,
    cursor::Cursor,
    game::{AppState, GameState},
    map::ActiveTeam,
    unit::{AttackEvent, Selected, Unit},
};

pub fn handle_open_choose_target(mut cursor_query: Query<&mut TextureAtlasSprite, With<Cursor>>) {
    let mut texture_atlas_sprite = cursor_query.single_mut().expect("Should be a cursor");
    texture_atlas_sprite.index = 1;
}

pub fn handle_exit_choose_target(mut cursor_query: Query<&mut TextureAtlasSprite, With<Cursor>>) {
    let mut texture_atlas_sprite = cursor_query.single_mut().expect("Should be a cursor");
    texture_atlas_sprite.index = 0;
}

pub fn handle_cursor_target_select(
    keyboard_input: Res<Input<KeyCode>>,
    cursor_query: Query<&Cell, With<Cursor>>,
    mut game_state: ResMut<State<AppState>>,
    mut attacking_unit_query: Query<Entity, (With<Selected>, With<Unit>)>,
    mut units_query: Query<(Entity, &mut Unit), Without<Selected>>,
    mut ev_attack: EventWriter<AttackEvent>,
    active_team: Res<ActiveTeam>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        info!("Game state: {:?}", game_state.current());
        let attacker_entity = attacking_unit_query
            .single_mut()
            .expect("Trying to attack a target without a unit selected!");
        let cursor_cell = cursor_query.single().expect("No Cursor found?!");

        for (defender_entity, unit) in units_query.iter_mut() {
            let unit_cell = &unit.location;
            let cursor_hovering = unit_cell.x == cursor_cell.x && unit_cell.y == cursor_cell.y;
            let is_enemy = unit.team != active_team.team;
            if is_enemy && cursor_hovering {
                ev_attack.send(AttackEvent(attacker_entity, defender_entity));

                game_state
                    .set(AppState::InGame(GameState::Browsing))
                    .expect("Problem changing state");
            }
        }
    }
}
