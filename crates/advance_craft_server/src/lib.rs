use std::net::SocketAddr;

use advance_craft_engine::{Command, CommandResult, ScenarioState, TeamID};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub type GameID = Uuid;

pub type PlayerID = SocketAddr; // Temporary 

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ClientToServer {
    CreateGame {
        // Map, rules etc.
    },
    ConnectToGame {
        game_id: GameID,
        team_id: TeamID,
    },
    InGameCommand {
        game_id: GameID,
        command: Command,
    },
    Test {
        message: String,
    },
}

#[derive(Serialize, Deserialize, Clone)]
pub enum ServerToClient {
    Error {
        message: String,
    },
    CreateGameResult {
        game_id: GameID,
        scenario_state: ScenarioState,
    },
    ConnectToGameResult {
        game_id: GameID,
        scenario_state: ScenarioState,
        team_id: TeamID,
    },
    // Combine these two?
    CommandResult {
        game_id: GameID,
        result: CommandResult,
    },
    GameUpdate {
        game_id: GameID,
        result: CommandResult,
    },
    Test {
        message: String,
    },
}
impl ServerToClient {
    pub fn new_error(message: String) -> ServerToClient {
        ServerToClient::Error { message }
    }
}
