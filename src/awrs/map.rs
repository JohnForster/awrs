use bevy::prelude::*;

use super::cell::*;
use super::constants::*;
use super::sprite_loading::{TerrainAtlas, UnitAtlas};
use super::unit::*;
use super::unit_loading::UnitHandle;
use super::unit_loading::UnitType;

pub struct GameMap {
    pub width: usize,
    pub height: usize,
}

#[derive(Bundle)]
struct GameMapBundle {
    game_map: GameMap,
    transform: Transform,
    global_transform: GlobalTransform,
}

pub struct ActiveTeam {
    pub team: Team,
}

// TODO Load sprites from json: https://github.com/serde-rs/json

// TODO: should probably move the part for instantiating units into unit.rs
pub fn build_map(
    mut commands: Commands,
    terrain_atlas: Res<TerrainAtlas>,
    unit_atlas: Res<UnitAtlas>,
    unit_handle: Res<UnitHandle>,
    unit_assets: Res<Assets<UnitType>>,
) {
    info!("Building Map");
    let game_map = vec![
        vec![0, 0, 0, 0, 0, 1],
        vec![0, 0, 0, 0, 1, 1],
        vec![0, 0, 0, 1, 1, 1],
        vec![0, 0, 1, 1, 1, 1],
        vec![0, 1, 1, 1, 1, 1],
        vec![1, 1, 1, 1, 1, 1],
    ];

    let infantry = unit_assets.get(&unit_handle.handle).unwrap();

    let units = vec![
        Unit {
            unit_type: 0,
            team: Team(0),
            location: Cell { x: 1, y: 1 },
            health: UnitHealth(infantry.max_health.clone()),
        },
        Unit {
            unit_type: 0,
            team: Team(1),
            location: Cell { x: 2, y: 1 },
            health: UnitHealth(infantry.max_health.clone()),
        },
    ];

    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    commands.insert_resource(ActiveTeam { team: Team(0) });

    commands
        .spawn()
        .insert(GameMap {
            height: game_map.len(),
            width: game_map[0].len(),
        })
        .insert(Transform {
            ..Default::default()
        })
        .insert(GlobalTransform {
            ..Default::default()
        })
        .with_children(|parent| {
            for (y, row) in game_map.iter().rev().enumerate() {
                for (x, &terrain_index) in row.iter().enumerate() {
                    parent.spawn_bundle(SpriteSheetBundle {
                        texture_atlas: terrain_atlas.atlas_handle.clone(),
                        sprite: TextureAtlasSprite::new(terrain_index),
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

    for (i, unit) in units.into_iter().enumerate() {
        let x = unit.location.x;
        let y = unit.location.y;
        commands.spawn_bundle(UnitBundle {
            id: UnitId(i),
            data: unit,
            sprite: SpriteSheetBundle {
                texture_atlas: unit_atlas.atlas_handle.clone(),
                sprite: TextureAtlasSprite::new(unit.team.0.clone()),
                transform: Transform::from_translation(Vec3::new(
                    x as f32 * TILE_SIZE,
                    y as f32 * TILE_SIZE,
                    1.0,
                )),
                ..Default::default()
            },
        });
    }
}
