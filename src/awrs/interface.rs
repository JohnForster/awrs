use bevy::prelude::*;

use super::engine::{Command, CommandResult, ScenarioState, Tile as EngineTile, Unit};
use super::tile::Tile;
use super::unit::{UnitHealth, UnitId};

pub struct ActionEvent(pub Action);

pub enum Action {
    Attack(Entity, Entity),
    Move(Entity, Vec<Tile>),
    EndTurn,
}

// Will need to add more detail once its clear what is needed from these result events.
pub enum ActionResult {
    AttackResult(Vec<(UnitId, UnitHealth)>), // Include ammo in this struct?
    MoveResult(Vec<Tile>),
    EndTurnResult(u32),
}

impl From<CommandResult> for ActionResultEvent {
    fn from(command_result: CommandResult) -> ActionResultEvent {
        let result = match command_result {
            CommandResult::Move { status, tiles } => ActionResult::MoveResult(
                tiles
                    .iter()
                    .map(|EngineTile { x, y }| Tile { x: *x, y: *y })
                    .collect(),
            ),
            CommandResult::Attack { status, unit_hp } => ActionResult::AttackResult(
                unit_hp
                    .iter()
                    .map(|(id, hp)| (UnitId(*id), UnitHealth(*hp)))
                    .collect(),
            ),
            CommandResult::EndTurn {
                status,
                new_active_team,
            } => ActionResult::EndTurnResult(new_active_team),
        };
        return ActionResultEvent(result);
    }
}

pub struct ActionResultEvent(pub ActionResult);

pub fn handle_action(
    ev_action: EventReader<ActionEvent>,
    mut ev_action_result: EventWriter<ActionResultEvent>,
    mut scenario_state: ResMut<ScenarioState>,
    mut q_units: Query<&Unit>,
) {
    for ActionEvent(action) in ev_action.iter() {
        let command = match action {
            Action::Attack(attacker_entity, defender_entity) => {
                let mut attacker = q_units
                    .get_mut(*attacker_entity)
                    .expect("Couldn't find attacker");
                let mut defender = q_units
                    .get_mut(*defender_entity)
                    .expect("Couldn't find defender");

                Command::Attack {
                    attacker_id: attacker.id,
                    defender_id: defender.id,
                }
            }

            Action::Move(entity, cells) => {
                let mut unit = q_units
                    .get_mut(*entity)
                    .expect("Couldn't find unit to move");

                Command::Move {
                    unit_id: unit.id,
                    tiles: cells
                        .iter()
                        .map(|tile| EngineTile {
                            x: tile.x,
                            y: tile.y,
                        })
                        .collect(),
                }
            }
            Action::EndTurn => Command::EndTurn,
        };

        let result = scenario_state.execute(command);
        ev_action_result.send(ActionResultEvent::from(result))
    }
}
