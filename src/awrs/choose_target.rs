use bevy::prelude::*;

use super::{
    cell::Cell,
    cursor::{ChangeCursorEvent, CursorStyle, SelectEvent},
    game::{AppState, GameState},
    map::ActiveTeam,
    unit::{AttackEvent, Selected, Unit},
};

pub fn open_target_selection(mut ev_change_cursor: EventWriter<ChangeCursorEvent>) {
    info!("Changed to Target Selection");
    ev_change_cursor.send(ChangeCursorEvent(CursorStyle::Target));
}

pub fn target_select(
    mut game_state: ResMut<State<AppState>>,
    mut attacking_unit_query: Query<Entity, (With<Selected>, With<Unit>)>,
    mut units_query: Query<(Entity, &mut Unit), Without<Selected>>,
    mut commands: Commands,
    active_team: Res<ActiveTeam>,
    mut ev_attack: EventWriter<AttackEvent>,
    mut ev_select: EventReader<SelectEvent>,
) {
    for SelectEvent(entity) in ev_select.iter() {
        // ! What happens if SelectEvent is triggered for a Selected unit?
        let (defender_entity, def_unit) =
            units_query.get_mut(*entity).expect("Unit doesn't exist?!");

        let is_enemy = def_unit.team != active_team.team;
        if !is_enemy {
            continue;
        }

        let attacker_entity = attacking_unit_query
            .single_mut()
            .expect("Trying to attack a target without a unit selected!");

        ev_attack.send(AttackEvent(attacker_entity, defender_entity));

        commands.entity(attacker_entity).remove::<Selected>();

        game_state
            .set(AppState::InGame(GameState::Browsing))
            .expect("Problem changing state");
    }
}
