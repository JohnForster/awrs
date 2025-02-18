use advance_craft_engine::TeamID;
use bevy::prelude::*;
use uuid::Uuid;

use super::scenario::ScenarioState;

#[derive(Event)]
pub struct StartGameEvent {
    pub scenario_state: ScenarioState,
    pub game_type: GameType,
}

#[derive(Resource, Debug, Clone)]
pub enum GameType {
    Online(OnlineMetadata),
    Offline,
}

#[derive(Debug, Clone, Copy)]
pub struct OnlineMetadata {
    pub game_id: Uuid,
    pub team_id: TeamID,
}

impl GameType {
    pub fn new_online(game_id: Uuid, team_id: TeamID) -> Self {
        GameType::Online(OnlineMetadata { game_id, team_id })
    }
}
