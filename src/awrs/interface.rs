use bevy::prelude::*;

use super::engine::{Command, CommandResult, ScenarioState, Tile as EngineTile, Unit, UnitHp};
use super::tile::Tile;
use super::unit::UnitId;

// Do we _need_ a wrapper here? Can the event be the enum?
pub struct ActionEvent(pub Action);

pub enum Action {
    Attack(Entity, Entity),
    Move { entity: Entity, tiles: Vec<Tile> },
    _EndTurn,
}

// Will need to add more detail once its clear what is needed from these result events.
pub enum ActionResultEvent {
    AttackResult(Vec<(UnitId, UnitHp)>), // Include ammo in this struct?
    MoveResult(Vec<Tile>),
    EndTurnResult(u32),
}

impl From<CommandResult> for ActionResultEvent {
    fn from(command_result: CommandResult) -> ActionResultEvent {
        match command_result {
            CommandResult::Move { status, tiles } => ActionResultEvent::MoveResult(
                tiles
                    .iter()
                    .map(|EngineTile { x, y }| Tile { x: *x, y: *y })
                    .collect(),
            ),
            CommandResult::Attack { status, unit_hp } => ActionResultEvent::AttackResult(
                unit_hp.iter().map(|(id, hp)| (UnitId(*id), *hp)).collect(),
            ),
            CommandResult::EndTurn {
                status,
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
    for ActionEvent(action) in ev_action.iter() {
        info!("Action event fired!");
        let command = match action {
            Action::Attack(attacker_entity, defender_entity) => {
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

            Action::Move { entity, tiles } => {
                let unit = q_units.get(*entity).expect("Unable to find unit");
                Command::Move {
                    unit_id: unit.0,
                    tiles: tiles
                        .iter()
                        .map(|tile| EngineTile {
                            x: tile.x,
                            y: tile.y,
                        })
                        .collect(),
                }
            }
            Action::_EndTurn => Command::EndTurn,
        };

        let result = scenario_state.execute(command);
        info!("Sending Action Result Event!");
        ev_action_result.send(ActionResultEvent::from(result))
    }
}
