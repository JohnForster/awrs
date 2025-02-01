use bevy::prelude::*;

use super::scenario::ScenarioState;

#[derive(Event)]
pub struct StartGameEvent {
    pub scenario_state: ScenarioState,
    pub online: bool,
    pub game_id: Option<String>,
}

#[derive(Resource, Debug)]
pub struct GameMetadata {
    pub _online: bool,
    pub _game_id: Option<String>,
}
