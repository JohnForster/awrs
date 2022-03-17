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

    let mut camera_bundle = OrthographicCameraBundle::new_2d();
    camera_bundle.orthographic_projection.scale /= 2.0;
    commands.spawn_bundle(camera_bundle);

    commands.spawn_bundle(UiCameraBundle::default());

    commands.insert_resource(ActiveTeam {
        team: scenario_state.active_team,
    });

    commands
        .spawn()
        .insert(GameMap {
            height: scenario_state.map.len(),
            width: scenario_state.map[0].len(),
        })
        .insert(Transform {
            ..Default::default()
        })
        .insert(GlobalTransform {
            ..Default::default()
        })
        .with_children(|parent| {
            for (y, row) in scenario_state.map.iter().rev().enumerate() {
                for (x, terrain_type) in row.iter().enumerate() {
                    parent.spawn_bundle(SpriteSheetBundle {
                        texture_atlas: terrain_atlas.atlas_handle.clone(),
                        sprite: TextureAtlasSprite::new(match terrain_type {
                            TerrainType::Water => 0,
                            TerrainType::Grass => 1,
                        }),
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
            .handle_map
            .get(&UnitType::from(unit.unit_type))
            .unwrap();

        let mut sprite = TextureAtlasSprite::new(1);
        sprite.flip_x = unit.team % 2 == 0;

        commands
            .spawn()
            .insert(UnitId(unit.id))
            .insert(Timer::from_seconds(0.8, true))
            .insert_bundle(SpriteSheetBundle {
                texture_atlas: texture_atlas.clone(),
                sprite,
                transform: Transform::from_translation(Vec3::new(
                    x as f32 * TILE_SIZE,
                    y as f32 * TILE_SIZE,
                    1.0,
                )),
                ..Default::default()
            })
            .with_children(|unit| {
                let transform = Transform::from_translation(Vec3::new(7.0, 7.0, 4.0));
                unit.spawn_bundle(SpriteSheetBundle {
                    texture_atlas: health_atlas.atlas_handle.clone(),
                    sprite: TextureAtlasSprite::new(9),
                    visibility: Visibility { is_visible: false },
                    transform,
                    ..Default::default()
                })
                .insert(HealthIndicator);
            });
    }
    commands.insert_resource(scenario_state);
}
