use advance_craft_engine::{TeamID, dev_helpers::new_scenario_state};

use crate::{Game, GameID, GameMap, OpenGameInfo, PlayerID, ServerToClient};

use advance_craft_engine::Command;

pub fn create_game(game_map: &GameMap) -> ServerToClient {
    let scenario_state = new_scenario_state();
    let game = Game::new(scenario_state);

    game_map
        .lock()
        .unwrap()
        .insert(game.id.clone(), game.clone());

    return ServerToClient::CreateGameResult {
        game_id: game.id,
        scenario_state: game.scenario_state,
    };
}

pub fn game_command(
    game_map: &GameMap,
    game_id: &GameID,
    command: Command,
    issuing_player: &PlayerID,
) -> ServerToClient {
    let mut binding = game_map.lock().unwrap();
    let game = match binding.get_mut(game_id) {
        None => return ServerToClient::new_error(format!("No game found with id {}", game_id)),
        Some(v) => v,
    };

    // TODO
    // Check if game is active
    // Check if it is the player's turn
    let (_, issuing_team) = game
        .players
        .iter()
        .find(|(player_id, _)| player_id == issuing_player)
        .unwrap();
    if game.scenario_state.active_team != *issuing_team {
        return ServerToClient::new_error("Not your turn".to_string());
    }

    if !game.started {
        game.started = true;
    }

    let result = game.scenario_state.execute(command);
    return ServerToClient::CommandResult {
        game_id: game.id,
        result,
    };
}

pub fn connect_to_game(
    game_map: &GameMap,
    game_id: &GameID,
    player_id: &PlayerID,
    team_id: TeamID,
) -> ServerToClient {
    let mut binding = game_map.lock().unwrap();
    let game = match binding.get_mut(game_id) {
        None => return ServerToClient::new_error(format!("No game found with id {}", game_id)),
        Some(v) => v,
    };

    if game.players.iter().any(|(existing_player, existing_team)| {
        existing_player == player_id || *existing_team == team_id
    }) {
        let err_msg = format!("Player {} or team {} already occupied ", player_id, team_id);
        return ServerToClient::new_error(err_msg);
    }

    game.players.push((*player_id, team_id));

    return ServerToClient::ConnectToGameResult {
        game_id: *game_id,
        scenario_state: game.scenario_state.clone(),
        team_id: team_id,
    };
}

pub fn list_games(game_map: &GameMap) -> ServerToClient {
    let binding = game_map.lock().unwrap();
    let games = binding
        .iter()
        .filter(|(_, game)| game.started == false)
        .map(|(id, game)| OpenGameInfo {
            id: id.clone(),
            scenario_state: game.scenario_state.clone(),
            players: game.players.clone(),
        })
        .collect();

    ServerToClient::GamesList { games }
}
