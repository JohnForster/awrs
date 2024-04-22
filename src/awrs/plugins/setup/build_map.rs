use bevy::prelude::*;

use crate::awrs::{
    constants::*,
    dev_helpers::{new_scenario_map, new_scenario_state},
    engine::TerrainType,
    resources::{
        atlases::{HealthAtlas, TerrainAtlas, UnitAtlases},
        map::{ActiveTeam, GameMap},
        unit::*,
    },
};

struct AnimationTimer {
    pub timer: Timer,
}

// TODO: should probably move the part for instantiating units elsewhere?
pub fn build_map(
    mut commands: Commands,
    terrain_atlas: Res<TerrainAtlas>,
    unit_atlases: Res<UnitAtlases>,
    health_atlas: Res<HealthAtlas>,
) {
    info!("Building map");
    let scenario_map = new_scenario_map();
    let scenario_state = new_scenario_state(scenario_map);

    let mut camera_bundle = Camera2dBundle::default();
    camera_bundle.projection.scale /= 2.0;
    commands.spawn(camera_bundle);

    // DEPRECATED?
    // commands.spawn_bundle(UiCameraBundle::default());

    commands.insert_resource(ActiveTeam {
        team: scenario_state.active_team,
    });

    commands
        .spawn((
            GameMap {
                height: scenario_state.map.len(),
                width: scenario_state.map[0].len(),
            },
            Transform {
                ..Default::default()
            },
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

    for unit in scenario_state.units.iter() {
        let x = unit.position.x;
        let y = unit.position.y;
        let texture_atlas = unit_atlases
            .atlas_map
            .get(&UnitType::from(unit.unit_type))
            .unwrap();

        let mut sprite = Sprite {
            flip_x: unit.team % 2 == 0,
            ..Default::default()
        };

        let timer = Timer::from_seconds(0.8, TimerMode::Repeating);

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
    commands.insert_resource(scenario_state);
}
