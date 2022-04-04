use crate::awrs::engine::{ScenarioMap, ScenarioState, Tile, Unit, UnitId, UnitType};

pub fn new_scenario_state(scenario_map: ScenarioMap) -> ScenarioState {
    let unit_data = vec![
        (UnitType::Marine, (1, 2), 0),
        (UnitType::Zergling, (3, 3), 1),
        (UnitType::Baneling, (4, 2), 1),
        (UnitType::Roach, (3, 1), 1),
    ];

    let units = unit_data
        .into_iter()
        .enumerate()
        .map(|(i, data)| {
            let (unit_type, location, team) = data;

            Unit {
                id: i as UnitId,
                unit_type: unit_type,
                position: Tile {
                    x: location.0,
                    y: location.1,
                },
                team,
                health: unit_type.value().max_health,
                has_moved: false,
                has_attacked: false,
            }
        })
        .collect();

    ScenarioState {
        map: scenario_map,
        units,
        active_team: 0,
        teams: vec![0, 1],
    }
}
