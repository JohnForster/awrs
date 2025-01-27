use crate::awrs::{
    dev_helpers::{new_scenario_map, new_scenario_state},
    advance_craft_engine::{Command, Tile},
};

pub fn _run_game() {
    let scenario_map = new_scenario_map();
    let mut scenario_state = new_scenario_state(scenario_map);

    let commands = vec![
        Command::Move {
            unit_id: 0,
            tiles: vec![
                Tile { x: 1, y: 2 },
                Tile { x: 2, y: 2 },
                Tile { x: 2, y: 3 },
            ],
        },
        Command::Attack {
            attacker_id: 0,
            defender_id: 1,
        },
        Command::EndTurn,
    ];

    for command in commands.into_iter() {
        scenario_state.execute(command);
    }

    println!("{:#?}", scenario_state);
}
