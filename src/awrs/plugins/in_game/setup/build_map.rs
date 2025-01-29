use advance_craft_engine::{dev_helpers::new_scenario_state, TerrainType};
use bevy::prelude::*;

use crate::awrs::{
    constants::*,
    resources::{
        animation::AnimationConfig,
        atlases::{CreepAtlas, HealthAtlas, StructureAtlases, TerrainAtlas, UnitAtlases},
        map::{ActiveTeam, GameMap},
        scenario::ScenarioState,
        unit::*,
    },
};

const SCALE: f32 = 2.0;

// TODO: should probably move the part for instantiating units elsewhere?
pub fn build_map(
    mut commands: Commands,
    terrain_atlas: Res<TerrainAtlas>,
    unit_atlases: Res<UnitAtlases>,
    structure_atlases: Res<StructureAtlases>,
    health_atlas: Res<HealthAtlas>,
    creep_atlas: Res<CreepAtlas>,
) {
    info!("Building map");
    let scenario_state = ScenarioState(new_scenario_state());

    let mut projection = OrthographicProjection::default_2d();
    projection.scale /= SCALE;
    commands.spawn((Camera2d::default(), projection));

    // DEPRECATED?
    // commands.spawn_bundle(UiCameraBundle::default());

    commands.insert_resource(ActiveTeam {
        team: scenario_state.active_team,
    });

    spawn_tiles(&mut commands, &scenario_state, &terrain_atlas);
    spawn_creep(&mut commands, &scenario_state, &creep_atlas);

    for unit in scenario_state.units.iter() {
        spawn_unit(&mut commands, unit, &unit_atlases, &health_atlas);
    }
    for structure in scenario_state.structures.iter() {
        spawn_structure(&mut commands, structure, &structure_atlases, &health_atlas);
    }
    commands.insert_resource(scenario_state);
}

#[derive(Component)]
struct Creep;

fn spawn_creep(
    commands: &mut Commands,
    scenario_state: &advance_craft_engine::ScenarioState,
    creep_atlas: &Res<CreepAtlas>,
) {
    commands
        .spawn((Transform::default(), Visibility::default(), Creep))
        .with_children(|parent| {
            for id in scenario_state.teams.iter() {
                if let Some(creep_map) = scenario_state.creep.0.get(&id) {
                    for (y, row) in creep_map.iter().enumerate() {
                        for (x, &has_creep) in row.iter().enumerate() {
                            if has_creep {
                                parent.spawn((
                                    Sprite {
                                        image: creep_atlas.texture.clone(),
                                        texture_atlas: Some(TextureAtlas {
                                            layout: creep_atlas.layout.clone(),
                                            index: get_creep_sprite(creep_map, x, y),
                                        }),
                                        ..Default::default()
                                    },
                                    Transform::from_translation(Vec3::new(
                                        x as f32 * TILE_SIZE,
                                        y as f32 * TILE_SIZE,
                                        0.0,
                                    )),
                                ));
                            }
                        }
                    }
                }
            }
        });
}

fn spawn_tiles(
    commands: &mut Commands,
    scenario_state: &advance_craft_engine::ScenarioState,
    terrain_atlas: &Res<TerrainAtlas>,
) {
    commands
        .spawn((
            GameMap {
                _height: scenario_state.map.len(),
                _width: scenario_state.map[0].len(),
            },
            Transform::default(),
            Visibility::default(),
        ))
        .with_children(|parent| {
            for (y, row) in scenario_state.map.iter().rev().enumerate() {
                for (x, terrain_type) in row.iter().enumerate() {
                    parent.spawn((
                        Sprite {
                            image: terrain_atlas.texture.clone(),
                            texture_atlas: Some(TextureAtlas {
                                layout: terrain_atlas.layout.clone(),
                                index: match terrain_type {
                                    TerrainType::Water => 0,
                                    TerrainType::Grass => 1,
                                },
                            }),
                            ..Default::default()
                        },
                        Transform::from_translation(Vec3::new(
                            x as f32 * TILE_SIZE,
                            y as f32 * TILE_SIZE,
                            0.0,
                        )),
                    ));
                }
            }
        });
}

fn spawn_unit(
    commands: &mut Commands,
    unit: &advance_craft_engine::Unit,
    unit_atlases: &Res<UnitAtlases>,
    health_atlas: &Res<HealthAtlas>,
) {
    let x = unit.position.x;
    let y = unit.position.y;
    let texture_atlas = unit_atlases
        .atlas_map
        .get(&UnitType::from(unit.unit_type))
        .unwrap();

    let animation_config = AnimationConfig::new(0, 3, 2);

    commands
        .spawn((
            UnitId(unit.id),
            Sprite {
                image: texture_atlas.texture.clone(),
                texture_atlas: Some(TextureAtlas {
                    layout: texture_atlas.layout.clone(),
                    index: 0,
                }),
                flip_x: unit.team % 2 == 0,
                ..Default::default()
            },
            Transform::from_translation(Vec3::new(x as f32 * TILE_SIZE, y as f32 * TILE_SIZE, 1.0)),
            animation_config,
        ))
        .with_children(|unit| {
            unit.spawn((
                HPIndicator,
                Sprite {
                    texture_atlas: Some(TextureAtlas {
                        layout: health_atlas.layout.clone(),
                        index: 9,
                    }),
                    image: health_atlas.texture.clone(),
                    ..Default::default()
                },
                Transform::from_translation(Vec3::new(7.0, 7.0, 4.0)),
                Visibility::Hidden,
            ));
        });
}

fn spawn_structure(
    commands: &mut Commands,
    structure: &advance_craft_engine::Structure,
    structure_atlases: &Res<StructureAtlases>,
    health_atlas: &Res<HealthAtlas>,
) {
    let x = structure.position.x;
    let y = structure.position.y;
    let texture_atlas = structure_atlases
        .atlas_map
        .get(&StructureType::from(structure.structure_type))
        .unwrap();

    commands
        .spawn((
            StructureId(structure.id),
            Sprite {
                image: texture_atlas.texture.clone(),
                texture_atlas: Some(TextureAtlas {
                    layout: texture_atlas.layout.clone(),
                    index: 0,
                }),
                ..Default::default()
            },
            Transform::from_translation(Vec3::new(x as f32 * TILE_SIZE, y as f32 * TILE_SIZE, 0.9)),
            AnimationConfig::new(0, 3, 2),
        ))
        .with_children(|unit| {
            unit.spawn((
                HPIndicator,
                Sprite {
                    image: health_atlas.texture.clone(),
                    texture_atlas: Some(TextureAtlas {
                        layout: health_atlas.layout.clone(),
                        index: 9,
                    }),
                    ..Default::default()
                },
                Transform::from_translation(Vec3::new(7.0, 7.0, 4.0)),
                Visibility::Hidden,
            ));
        });
}

struct Coord {
    x: i32,
    y: i32,
}

impl Coord {
    fn new(x: i32, y: i32) -> Self {
        Coord { x, y }
    }
}

fn get_creep_sprite(creep_map: &Vec<Vec<bool>>, x: usize, y: usize) -> usize {
    let x = x as i32;
    let y = y as i32;
    let up = Coord::new(x, y + 1);
    let down = Coord::new(x, y - 1);
    let left = Coord::new(x - 1, y);
    let right = Coord::new(x + 1, y);

    let u_d_l_r = [up, down, left, right].map(|Coord { x, y }| {
        if x < 0 || y < 0 {
            return false;
        }

        let (x, y) = (x as usize, y as usize);

        if let Some(row) = creep_map.get(y) {
            if let Some(has_creep) = row.get(x) {
                return *has_creep;
            }
        }
        return false;
    });

    match u_d_l_r {
        [false, true, false, true] => 0,
        [false, true, true, true] => 1,
        [false, true, true, false] => 2,
        [true, true, false, true] => 3,
        [true, true, true, true] => 4,
        [true, true, true, false] => 5,
        [true, false, false, true] => 6,
        [true, false, true, true] => 7,
        [true, false, true, false] => 8,
        _ => 4,
    }
}
