use std::{
    collections::HashMap,
    env,
    io::Error as IoError,
    net::SocketAddr,
    sync::{Arc, Mutex},
};

use advance_craft_engine::{Command, CommandResult, CommandStatus};
use futures_channel::mpsc::{TrySendError, UnboundedSender, unbounded};
use futures_util::{StreamExt, future, pin_mut, stream::TryStreamExt};

use serde::{Deserialize, Serialize};
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::tungstenite::protocol::Message;

type Tx = UnboundedSender<Message>;
type PeerMap = Arc<Mutex<HashMap<SocketAddr, Tx>>>;

#[derive(Debug, Serialize, Deserialize)]
pub struct IncomingMessage {
    command: Command,
    game_id: String,
    game_hash: String,
}

#[derive(Serialize, Deserialize)]

pub struct OutgoingMessage {
    result: CommandResult,
    game_hash: String,
}

#[tokio::main]
async fn main() -> Result<(), IoError> {
    let addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:8080".to_string());

    let state = PeerMap::new(Mutex::new(HashMap::new()));

    // Create the event loop and TCP listener we'll accept connections on.
    let try_socket = TcpListener::bind(&addr).await;
    let listener = try_socket.expect("Failed to bind");
    println!("Listening on: {}", addr);

    // Let's spawn the handling of each connection in a separate task.
    while let Ok((stream, addr)) = listener.accept().await {
        tokio::spawn(handle_connection(state.clone(), stream, addr));
    }

    Ok(())
}

async fn handle_connection(peer_map: PeerMap, raw_stream: TcpStream, addr: SocketAddr) {
    println!("Incoming TCP connection from: {}", addr);

    let ws_stream = tokio_tungstenite::accept_async(raw_stream)
        .await
        .expect("Error during the websocket handshake occurred");
    println!("WebSocket connection established: {}", addr);

    // Insert the write part of this peer to the peer map.
    let (tx, rx) = unbounded();
    peer_map.lock().unwrap().insert(addr, tx);

    let (out_stream, in_stream) = ws_stream.split();

    let broacast_ft = in_stream.try_for_each(|msg| handle_incoming(&peer_map, addr, msg));
    let receive_ft = rx.map(Ok).forward(out_stream);

    pin_mut!(broacast_ft, receive_ft);
    future::select(broacast_ft, receive_ft).await;

    println!("{} disconnected", &addr);
    peer_map.lock().unwrap().remove(&addr);
}

fn handle_incoming(
    peer_map: &Arc<Mutex<HashMap<SocketAddr, UnboundedSender<Message>>>>,
    addr: SocketAddr,
    msg: Message,
) -> future::Ready<Result<(), tokio_tungstenite::tungstenite::Error>> {
    log_message(addr, &msg);
    match parse_incoming_message(&msg) {
        Ok(incoming_message) => {
            println!("command: {:?}", incoming_message);
            let _ = send_response(&addr, peer_map);
        }
        Err(err) => println!("{:?}", err),
    }
    broadcast_to_others(addr, msg, peer_map);
    future::ok(())
}

fn log_message(addr: SocketAddr, msg: &Message) {
    println!(
        "Received a message from {}: {}",
        addr,
        msg.to_text().unwrap()
    );
}

fn broadcast_to_others(addr: SocketAddr, msg: Message, peer_map: &PeerMap) {
    let peers = peer_map.lock().unwrap();
    // We want to broadcast the message to everyone except ourselves.
    let broadcast_recipients = peers
        .iter()
        .filter(|(peer_addr, _)| peer_addr != &&addr)
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

fn parse_incoming_message(msg: &Message) -> Result<IncomingMessage, IncomingMsgParseErr> {
    match msg.to_text() {
        Ok(str) => serde_json::from_str::<IncomingMessage>(str).map_err(|serde_err| {
            println!("serde_err: {:?}", serde_err);
            IncomingMsgParseErr::SerdeErr
        }),
        Err(_) => Err(IncomingMsgParseErr::TextConversionErr),
    }
}

fn send_response(addr: &SocketAddr, peer_map: &PeerMap) -> Result<(), TrySendError<Message>> {
    let peers = peer_map.lock().unwrap();
    let recp = peers.get(addr).unwrap();
    let outgoing_message = OutgoingMessage {
        result: CommandResult::EndTurn {
            status: CommandStatus::Ok,
            new_active_team: 1,
        },
        game_hash: "abcde".to_string(),
    };

    recp.unbounded_send(Message::text(
        serde_json::to_string(&outgoing_message).unwrap(),
    ))
}
