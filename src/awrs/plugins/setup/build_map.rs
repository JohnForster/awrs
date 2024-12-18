use bevy::prelude::*;

use crate::awrs::{
    constants::*,
    dev_helpers::{new_scenario_map, new_scenario_state},
    engine::TerrainType,
    resources::{
        animation::AnimationConfig,
        atlases::{CreepAtlas, HealthAtlas, StructureAtlases, TerrainAtlas, UnitAtlases},
        map::{ActiveTeam, GameMap},
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
    let scenario_map = new_scenario_map();
    let scenario_state = new_scenario_state(scenario_map);

    let mut camera_bundle = Camera2dBundle::default();
    camera_bundle.projection.scale /= SCALE;
    commands.spawn(camera_bundle);

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
    scenario_state: &crate::awrs::engine::ScenarioState,
    creep_atlas: &Res<CreepAtlas>,
) {
    commands
        .spawn((SpatialBundle::default(), Creep))
        .with_children(|parent| {
            for id in scenario_state.teams.iter() {
                if let Some(creep_map) = scenario_state.creep.0.get(&id) {
                    for (y, row) in creep_map.iter().enumerate() {
                        for (x, &has_creep) in row.iter().enumerate() {
                            if has_creep {
                                let atlas = TextureAtlas {
                                    layout: creep_atlas.layout.clone(),
                                    index: get_creep_sprite(creep_map, x, y),
                                };
                                parent.spawn(SpriteSheetBundle {
                                    texture: creep_atlas.texture.clone(),
                                    atlas,
                                    transform: Transform::from_translation(Vec3::new(
                                        x as f32 * TILE_SIZE,
                                        y as f32 * TILE_SIZE,
                                        0.0,
                                    )),
                                    ..Default::default()
                                });
                            }
                        }
                    }
                }
            }
        });
}

fn spawn_tiles(
    commands: &mut Commands,
    scenario_state: &crate::awrs::engine::ScenarioState,
    terrain_atlas: &Res<TerrainAtlas>,
) {
    commands
        .spawn((
            GameMap {
                height: scenario_state.map.len(),
                width: scenario_state.map[0].len(),
            },
            SpatialBundle::default(),
        ))
        .with_children(|parent| {
            for (y, row) in scenario_state.map.iter().rev().enumerate() {
                for (x, terrain_type) in row.iter().enumerate() {
                    let atlas = TextureAtlas {
                        layout: terrain_atlas.layout.clone(),
                        index: match terrain_type {
                            TerrainType::Water => 0,
                            TerrainType::Grass => 1,
                        },
                    };
                    parent.spawn(SpriteSheetBundle {
                        texture: terrain_atlas.texture.clone(),
                        atlas,
                        transform: Transform::from_translation(Vec3::new(
                            x as f32 * TILE_SIZE,
                            y as f32 * TILE_SIZE,
                            0.0,
                        )),
                        ..Default::default()
                    });
                }
            }
        });
}

fn spawn_unit(
    commands: &mut Commands,
    unit: &crate::awrs::engine::Unit,
    unit_atlases: &Res<UnitAtlases>,
    health_atlas: &Res<HealthAtlas>,
) {
    let x = unit.position.x;
    let y = unit.position.y;
    let texture_atlas = unit_atlases
        .atlas_map
        .get(&UnitType::from(unit.unit_type))
        .unwrap();

    let sprite = Sprite {
        flip_x: unit.team % 2 == 0,
        ..Default::default()
    };

    let animation_config = AnimationConfig::new(0, 3, 2);

    commands
        .spawn((
            UnitId(unit.id),
            SpriteSheetBundle {
                texture: texture_atlas.texture.clone(),
                atlas: TextureAtlas {
                    layout: texture_atlas.layout.clone(),
                    index: 0,
                },
                sprite,
                transform: Transform::from_translation(Vec3::new(
                    x as f32 * TILE_SIZE,
                    y as f32 * TILE_SIZE,
                    1.0,
                )),
                ..Default::default()
            },
            animation_config,
        ))
        .with_children(|unit| {
            let transform = Transform::from_translation(Vec3::new(7.0, 7.0, 4.0));
            let atlas = TextureAtlas {
                layout: health_atlas.layout.clone(),
                index: 9,
            };
            unit.spawn((
                HealthIndicator,
                SpriteSheetBundle {
                    atlas,
                    texture: health_atlas.texture.clone(),
                    sprite: Sprite::default(),
                    visibility: Visibility::Hidden,
                    transform,
                    ..Default::default()
                },
            ));
        });
}

fn spawn_structure(
    commands: &mut Commands,
    structure: &crate::awrs::engine::Structure,
    structure_atlases: &Res<StructureAtlases>,
    health_atlas: &Res<HealthAtlas>,
) {
    let x = structure.position.x;
    let y = structure.position.y;
    let texture_atlas = structure_atlases
        .atlas_map
        .get(&StructureType::from(structure.structure_type))
        .unwrap();

    let sprite = Sprite {
        ..Default::default()
    };

    let animation_config = AnimationConfig::new(0, 3, 2);

    commands
        .spawn((
            StructureId(structure.id),
            SpriteSheetBundle {
                texture: texture_atlas.texture.clone(),
                atlas: TextureAtlas {
                    layout: texture_atlas.layout.clone(),
                    index: 0,
                },
                sprite,
                transform: Transform::from_translation(Vec3::new(
                    x as f32 * TILE_SIZE,
                    y as f32 * TILE_SIZE,
                    0.9,
                )),
                ..Default::default()
            },
            animation_config,
        ))
        .with_children(|unit| {
            let transform = Transform::from_translation(Vec3::new(7.0, 7.0, 4.0));
            let atlas = TextureAtlas {
                layout: health_atlas.layout.clone(),
                index: 9,
            };
            unit.spawn((
                HealthIndicator,
                SpriteSheetBundle {
                    atlas,
                    texture: health_atlas.texture.clone(),
                    sprite: Sprite::default(),
                    visibility: Visibility::Hidden,
                    transform,
                    ..Default::default()
                },
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
