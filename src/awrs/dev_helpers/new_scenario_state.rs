use crate::awrs::engine::{
    Creep, ScenarioMap, ScenarioState, Structure, StructureId, StructureType, Tile, Unit, UnitId,
    UnitType,
};

pub fn new_scenario_state(scenario_map: ScenarioMap) -> ScenarioState {
    let units = create_units();
    let structures = create_structures();
    let creep = create_creep(&scenario_map);

    ScenarioState {
        map: scenario_map,
        units,
        structures,
        active_team: 0,
        teams: vec![0, 1],
        creep,
    }
}

fn create_structures() -> Vec<Structure> {
    let structure_data = vec![
        (StructureType::CommandCentre, (1, 2), 0),
        (StructureType::CommandCentre, (1, 4), 0),
        (StructureType::Hatchery, (5, 2), 1),
        (StructureType::Hatchery, (7, 4), 1),
    ];

    let structures = structure_data
        .into_iter()
        .enumerate()
        .map(|(i, data)| {
            let (structure_type, location, team) = data;

            Structure {
                id: i as StructureId,
                structure_type,
                position: Tile {
                    x: location.0,
                    y: location.1,
                },
                team,
                health: structure_type.value().max_health,
            }
        })
        .collect();

    return structures;
}

fn create_units() -> Vec<Unit> {
    let unit_data = vec![
        (UnitType::Marine, (2, 1), 0),
        (UnitType::Marine, (2, 3), 0),
        (UnitType::Marine, (3, 4), 0),
        (UnitType::SiegeTank, (0, 2), 0),
        (UnitType::SiegeTank, (1, 3), 0),
        (UnitType::Zergling, (4, 1), 1),
        (UnitType::Zergling, (4, 4), 1),
        (UnitType::Zergling, (5, 3), 1),
        (UnitType::Baneling, (4, 2), 1),
        (UnitType::Roach, (5, 4), 1),
        (UnitType::Roach, (5, 5), 1),
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
    units
}

pub fn create_creep(map: &ScenarioMap) -> Creep {
    let mut creep = Creep::empty(2, map);

    let mut map = vec![
        vec![false, false, false, false, false, true, true, true, true],
        vec![false, false, false, false, true, true, true, true, true],
        vec![false, false, false, true, true, true, true, true, true],
        vec![false, false, false, true, true, true, true, true, true],
        vec![false, false, false, true, true, true, true, true, false],
        vec![false, false, false, false, true, true, true, false, false],
    ];
    map.reverse();

    creep.set(1, map);

    return creep;
}
