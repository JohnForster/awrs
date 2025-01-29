use std::{
    collections::HashMap,
    env,
    io::Error as IoError,
    net::SocketAddr,
    sync::{Arc, Mutex},
};

use advance_craft_engine::{
    Command, CommandResult, ScenarioState, TeamID, dev_helpers::new_scenario_state,
};
use advance_craft_server::*;
use futures_channel::mpsc::{TrySendError, UnboundedSender, unbounded};
use futures_util::{StreamExt, future, pin_mut, stream::TryStreamExt};

use serde::{Deserialize, Serialize};
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::tungstenite::protocol::Message;
use uuid::Uuid;

type Tx = UnboundedSender<Message>;
type PeerMap = Arc<Mutex<HashMap<SocketAddr, Tx>>>;
type GameMap = Arc<Mutex<HashMap<GameID, Game>>>;
type PlayerMap = Arc<Mutex<HashMap<PlayerID, Player>>>;

struct Player {
    id: PlayerID,
    socket_addr: SocketAddr,
    game_id: GameID, // For now, assume player can only be in one game at a time.
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Game {
    id: GameID,
    scenario_state: ScenarioState,
    players: Vec<(PlayerID, TeamID)>,
    started: bool,
    completed: bool,
    history: Vec<Command>,
}

impl Game {
    pub fn new(scenario_state: ScenarioState) -> Game {
        Game {
            id: Uuid::new_v4(),
            scenario_state,
            players: vec![],
            started: false,
            completed: false,
            history: vec![],
        }
    }
}
// ? ----------------------------------

#[tokio::main]
async fn main() -> Result<(), IoError> {
    let addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:8080".to_string());

    // Combine into one State struct?
    let state = PeerMap::new(Mutex::new(HashMap::new()));
    let games = GameMap::new(Mutex::new(HashMap::new()));
    let players = PlayerMap::new(Mutex::new(HashMap::new()));

    // Create the event loop and TCP listener we'll accept connections on.
    let try_socket = TcpListener::bind(&addr).await;
    let listener = try_socket.expect("Failed to bind");
    println!("Listening on: {}", addr);

    // Let's spawn the handling of each connection in a separate task.
    while let Ok((stream, addr)) = listener.accept().await {
        tokio::spawn(handle_connection(
            state.clone(),
            players.clone(),
            games.clone(),
            stream,
            addr,
        ));
    }

    Ok(())
}

async fn handle_connection(
    peer_map: PeerMap,
    player_map: PlayerMap,
    game_map: GameMap,
    raw_stream: TcpStream,
    addr: SocketAddr,
) {
    println!("Incoming TCP connection from: {}", addr);

    let ws_stream = tokio_tungstenite::accept_async(raw_stream)
        .await
        .expect("Error during the websocket handshake occurred");
    println!("WebSocket connection established: {}", addr);

    // Insert the write part of this peer to the peer map.
    let (tx, rx) = unbounded();
    peer_map.lock().unwrap().insert(addr, tx);

    let (out_stream, in_stream) = ws_stream.split();

    let broacast_ft = in_stream
        .try_for_each(|msg| handle_incoming(&peer_map, &player_map, &game_map, &addr, msg));
    let receive_ft = rx.map(Ok).forward(out_stream);

    pin_mut!(broacast_ft, receive_ft);
    future::select(broacast_ft, receive_ft).await;

    println!("{} disconnected", &addr);
    peer_map.lock().unwrap().remove(&addr);
}

fn handle_incoming(
    peer_map: &PeerMap,
    _player_map: &PlayerMap,
    game_map: &GameMap,
    addr: &SocketAddr,
    msg: Message,
) -> future::Ready<Result<(), tokio_tungstenite::tungstenite::Error>> {
    log_message(addr, &msg);
    let outgoing_message = match parse_incoming_message(&msg) {
        Ok(ClientToServer::CreateGame {}) => handle_create_game(&game_map),
        Ok(ClientToServer::ConnectToGame { game_id, team_id }) => {
            handle_connect_to_game(&game_map, &game_id, addr, team_id)
        }
        Ok(ClientToServer::InGameCommand { game_id, command }) => {
            handle_game_command(&game_map, &game_id, command, addr)
        }
        Ok(ClientToServer::Test { message }) => ServerToClient::Test {
            message: format!("message received: {}", message),
        },
        Err(err) => {
            println!("{:?}", err);
            ServerToClient::Error {
                message: "Error parsing message".to_string(),
            }
        }
        _ => ServerToClient::new_error("Not yet implemented".to_string()),
    };

    send_response(&outgoing_message, addr, peer_map).unwrap();
    if let ServerToClient::CommandResult { result, game_id } = outgoing_message {
        update_other_players(addr, &result, game_map, &game_id, peer_map);
    }
    future::ok(())
}

fn update_other_players(
    issuing_player: &PlayerID,
    command_result: &CommandResult,
    game_map: &GameMap,
    game_id: &GameID,
    peer_map: &PeerMap,
) {
    let binding = game_map.lock().unwrap();
    let game = binding.get(game_id).unwrap();
    for (player_id, _) in game.players.iter() {
        if player_id == issuing_player {
            continue;
        }
        let message = ServerToClient::GameUpdate {
            game_id: game_id.clone(),
            result: command_result.clone(),
        };
        send_response(&message, player_id, peer_map).unwrap();
    }
}

fn handle_create_game(game_map: &GameMap) -> ServerToClient {
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

fn handle_game_command(
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

fn handle_connect_to_game(
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

fn log_message(addr: &SocketAddr, msg: &Message) {
    println!(
        "Received a message from {}: {}",
        addr,
        msg.to_text().unwrap()
    );
}

fn _broadcast_to_others(addr: &SocketAddr, msg: Message, peer_map: &PeerMap) {
    let peers = peer_map.lock().unwrap();
    // We want to broadcast the message to everyone except ourselves.
    let broadcast_recipients = peers
        .iter()
        .filter(|(peer_addr, _)| peer_addr != &addr)
        .map(|(_, ws_sink)| ws_sink);

    for recp in broadcast_recipients {
        recp.unbounded_send(msg.clone()).unwrap();
    }
}

#[derive(Debug)]
enum IncomingMsgParseErr {
    TextConversionErr,
    SerdeErr,
}

fn parse_incoming_message(msg: &Message) -> Result<ClientToServer, IncomingMsgParseErr> {
    match msg.to_text() {
        Ok(str) => serde_json::from_str::<ClientToServer>(str).map_err(|serde_err| {
            println!("serde_err: {:?}", serde_err);
            IncomingMsgParseErr::SerdeErr
        }),
        Err(_) => Err(IncomingMsgParseErr::TextConversionErr),
    }
}

fn send_response(
    message: &ServerToClient,
    addr: &SocketAddr,
    peer_map: &PeerMap,
) -> Result<(), TrySendError<Message>> {
    let peers = peer_map.lock().unwrap();
    let recp = peers.get(addr).unwrap();

    recp.unbounded_send(Message::text(serde_json::to_string(message).unwrap()))
}
