use crate::awrs::engine::{ScenarioMap, TerrainType};

pub fn new_scenario_map() -> ScenarioMap {
    let mut number_terrain_map = vec![
        vec![0, 1, 1, 1, 1],
        vec![1, 1, 1, 1, 1],
        vec![1, 1, 1, 1, 1],
        vec![1, 1, 1, 1, 1],
        vec![1, 1, 1, 1, 0],
    ];
    number_terrain_map.reverse();

    let landscape: ScenarioMap = number_terrain_map
        .iter()
        .map(|row| {
            row.iter()
                .map(|n| match n {
                    0 => TerrainType::Water,
                    1 => TerrainType::Grass,
                    _ => panic!("No terrain implemented for index {}", n),
                })
                .collect()
        })
        .collect();

    return landscape;
}
