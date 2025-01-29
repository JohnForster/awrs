use advance_craft_engine::{Command, CommandResult, Tile as EngineTile};
use bevy::prelude::*;
use uuid::Uuid;

use crate::awrs::{
    constants::TILE_SIZE,
    resources::{
        action_event::{Action, ActionEvent, ActionResultEvent, Attack},
        client::SendWebsocketMessageEvent,
        scenario::ScenarioState,
        state::{AppState, GameState},
        tile::Tile,
        unit::{DamageEvent, HPIndicator, Selected, UnitId},
    },
};

use advance_craft_server::ClientToServer;

use super::cursor::Cursor;

pub struct InterfacePlugin;

impl Plugin for InterfacePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ActionEvent>()
            .add_event::<ActionResultEvent>()
            .add_systems(Update, handle_action.run_if(in_state(AppState::InGame)))
            .add_systems(
                Update,
                (handle_attack_result, handle_damage, move_result)
                    .run_if(in_state(AppState::InGame))
                    .after(handle_action),
            );
    }
}

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
    mut ev_client: EventWriter<SendWebsocketMessageEvent>,
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

        let result = scenario_state.execute(command.clone());
        info!("{:?}", result);

        info!("Sending message to server!");

        let message = ClientToServer::InGameCommand {
            game_id: Uuid::new_v4(),
            command,
        };
        info!("Sending Action Result Event! ({:?})", message);
        ev_client.send(SendWebsocketMessageEvent::from(message));

        ev_action_result.send(ActionResultEvent::from(result));
    }
}

pub fn handle_damage(
    mut ev_damage: EventReader<DamageEvent>,
    mut units_query: Query<(&UnitId, &Children)>,
    mut q_hp_indicator: Query<(&mut Sprite, &mut Visibility), With<HPIndicator>>,
    mut commands: Commands,
    scenario_state: Res<ScenarioState>,
) {
    for DamageEvent { entity, new_hp } in ev_damage.read() {
        info!("Handling Damage Event");
        let (unit_id, children) = units_query
            .get_mut(*entity)
            .expect("Could not find unit to damage");

        for &child in children.iter() {
            let Ok((mut sprite, mut visibility)) = q_hp_indicator.get_mut(child) else {
                continue;
            };
            info!("Updating health indicator");
            match scenario_state.get_unit(unit_id.0) {
                Some(unit) => {
                    let max_health = unit.unit_type.value().max_health;
                    println!("max_health: {:?}", max_health);
                    let health_percent = new_hp / max_health;
                    let ceil_health = (health_percent * 10.0).ceil().max(0.0) as usize;
                    info!("new_hp: {:?}, ceil_health: {:?}", new_hp, ceil_health);
                    let Some(atlas) = &mut sprite.texture_atlas else {
                        continue;
                    };
                    if ceil_health == 0 {
                        commands.entity(*entity).despawn_recursive()
                    } else if ceil_health < 10 {
                        *visibility = Visibility::Visible;
                        atlas.index = ceil_health - 1;
                    }
                }
                None => commands.entity(*entity).despawn_recursive(),
            }
        }
    }
}

pub fn handle_attack_result(
    mut q_units: Query<(Entity, &UnitId, &mut Sprite)>,
    mut ev_action_result: EventReader<ActionResultEvent>,
    mut ev_damage: EventWriter<DamageEvent>,
    scenario_state: Res<ScenarioState>,
) {
    for action_result in ev_action_result.read() {
        if let ActionResultEvent::AttackResult(damaged_units) = action_result {
            for (id, hp) in damaged_units {
                for (entity, unit_id, _) in q_units.iter_mut() {
                    if unit_id.0 == id.0 {
                        info!("Sending DamageEvent");
                        ev_damage.send(DamageEvent {
                            entity,
                            new_hp: *hp,
                        });
                    }
                }
            }
        }

        const GRAY: Srgba = bevy::color::palettes::css::GRAY;
        for (_, UnitId(unit_id), mut sprite) in q_units.iter_mut() {
            if scenario_state.unit_cannot_act(unit_id) {
                sprite.color = GRAY.into();
            }
        }
    }
}
pub fn move_result(
    mut ev_move_result: EventReader<ActionResultEvent>,
    mut next_state: ResMut<NextState<GameState>>,
    mut q: ParamSet<(
        Query<&mut Transform, With<Selected>>,
        Query<&mut Transform, With<Cursor>>,
    )>,
) {
    for action_result in ev_move_result.read() {
        if let ActionResultEvent::MoveResult(tiles) = action_result {
            info!("Executing move_result");
            if let Some(location) = tiles.last() {
                info!("Moving unit...");

                let mut unit_query = q.p0();
                let mut unit_transform = unit_query.single_mut();
                unit_transform.translation.x = location.x as f32 * TILE_SIZE;
                unit_transform.translation.y = location.y as f32 * TILE_SIZE;

                let mut cursor_query = q.p1();
                let mut cursor_transform = cursor_query.single_mut();
                cursor_transform.translation.x = location.x as f32 * TILE_SIZE;
                cursor_transform.translation.y = location.y as f32 * TILE_SIZE;
            } else {
            }

            next_state.set(GameState::Browsing);
        }
    }
}
