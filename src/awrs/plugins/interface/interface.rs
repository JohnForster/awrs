use bevy::prelude::*;

use crate::awrs::{
    engine::{Command, CommandResult, ScenarioState, Tile as EngineTile},
    resources::{
        action_event::{Action, ActionEvent, ActionResultEvent, Attack},
        tile::Tile,
        unit::UnitId,
    },
};

impl From<&Tile> for EngineTile {
    fn from(tile: &Tile) -> EngineTile {
        EngineTile {
            x: tile.x,
            y: tile.y,
        }
    }
}

impl From<CommandResult> for ActionResultEvent {
    fn from(command_result: CommandResult) -> ActionResultEvent {
        match command_result {
            CommandResult::Move { status: _, tiles } => ActionResultEvent::MoveResult(
                tiles
                    .iter()
                    .map(|EngineTile { x, y }| Tile { x: *x, y: *y })
                    .collect(),
            ),
            CommandResult::Attack {
                status: _,
                unit_hp_changes: unit_hp,
            } => ActionResultEvent::AttackResult(
                unit_hp.iter().map(|(id, hp)| (UnitId(*id), *hp)).collect(),
            ),
            CommandResult::AttackGround {
                status: _,
                unit_hp_changes: unit_hp,
            } => ActionResultEvent::AttackResult(
                unit_hp.iter().map(|(id, hp)| (UnitId(*id), *hp)).collect(),
            ),
            CommandResult::EndTurn {
                status: _,
                new_active_team,
            } => ActionResultEvent::EndTurnResult(new_active_team),
        }
    }
}

pub fn handle_action(
    mut ev_action: EventReader<ActionEvent>,
    mut ev_action_result: EventWriter<ActionResultEvent>,
    mut scenario_state: ResMut<ScenarioState>,
    q_units: Query<&UnitId>,
) {
    for ActionEvent(action) in ev_action.read() {
        info!("Action event ({:?}) recieved", action);
        let command = match action {
            Action::Attack(attack) => match attack {
                Attack::Unit(attacker_entity, defender_entity) => {
                    let &UnitId(attacker_id) = q_units
                        .get(*attacker_entity)
                        .expect("Couldn't find attacker");

                    let &UnitId(defender_id) = q_units
                        .get(*defender_entity)
                        .expect("Couldn't find defender");

                    Command::Attack {
                        attacker_id,
                        defender_id,
                    }
                }
                Attack::Ground(attacker_entity, tile) => {
                    let &UnitId(attacker_id) = q_units
                        .get(*attacker_entity)
                        .expect("Couldn't find attacker");
                    Command::AttackGround {
                        attacker_id,
                        tile: EngineTile::from(tile),
                    }
                }
            },

            Action::Move { entity, tiles } => {
                let unit = q_units.get(*entity).expect("Unable to find unit");
                Command::Move {
                    unit_id: unit.0,
                    tiles: tiles.iter().map(|tile| EngineTile::from(tile)).collect(),
                }
            }
            Action::EndTurn => Command::EndTurn,
        };

        info!("Sending Action Result Event! ({:?})", command);
        let result = scenario_state.execute(command);
        info!("{:?}", result);
        ev_action_result.send(ActionResultEvent::from(result));
    }
}
