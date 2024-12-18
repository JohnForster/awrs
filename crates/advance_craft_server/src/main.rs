use std::{
    collections::HashMap,
    convert::Infallible,
    sync::{Arc, Mutex},
};

use advance_craft_engine::{CommandResult, ScenarioState, new_scenario_map, new_scenario_state};
use tokio::sync::mpsc;
use warp::{Error, reject::Rejection, reply::Reply};
use warp::{Filter, filters::ws::Message};

struct Client {
    pub user_id: usize,
    pub game_id: Option<String>,
    pub sender: Option<mpsc::UnboundedSender<std::result::Result<Message, warp::Error>>>,
}

struct Game {
    pub game_id: String,
    pub scenario_state: ScenarioState,
}

type Result<T> = std::result::Result<T, Rejection>;
type Clients = Arc<Mutex<HashMap<String, Client>>>;
type Games = Arc<Mutex<HashMap<String, Game>>>;

async fn health_handler() -> Result<impl Reply> {
    Ok(warp::reply())
}

#[derive(serde::Deserialize, serde::Serialize)]
struct RegisterRequest;

async fn register_handler(body: RegisterRequest, clients: Clients) -> Result<impl Reply> {
    Ok(warp::reply())
}

async fn unregister_handler(id: String, clients: Clients) -> Result<impl Reply> {
    Ok(warp::reply())
}

pub struct Event {
    game_id: String,
    user_id: String,
    event: CommandResult,
}

#[tokio::main]
async fn main() {
    let mut clients: Clients = Arc::new(Mutex::new(HashMap::new()));
    let mut games: Games = Arc::new(Mutex::new(HashMap::new()));

    let health_route = warp::path!("health").and_then(health_handler);

    let register = warp::path("register");
    let register_routes = register
        .and(warp::post())
        .and(warp::body::json())
        .and(with_clients(clients.clone()))
        .and_then(register_handler)
        .or(register
            .and(warp::delete())
            .and(warp::path::param())
            .and(with_clients(clients.clone()))
            .and_then(unregister_handler));

    let cors = warp::cors().allow_any_origin();
    let routes = health_route.or(register_routes).with(cors);
    // .or(ws_route)
    // .or(publish)

    let a = warp::serve(routes);
    a.run(([127, 0, 0, 1], 8000)).await;

    let scenario_map = new_scenario_map();
    let scenario_state = new_scenario_state(scenario_map);
}

fn with_clients(clients: Clients) -> impl Filter<Extract = (Clients,), Error = Infallible> + Clone {
    warp::any().map(move || clients.clone())
}

async fn not_found_handler(_: String) -> Result<impl Reply> {
    Ok("Not found")
}
