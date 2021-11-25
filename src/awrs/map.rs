use bevy::prelude::*;

use super::cell::*;
use super::constants::*;
use super::sprite_loading::{TerrainAtlas, UnitAtlas};
use super::unit::*;

// TODO Load sprites from json: https://github.com/serde-rs/json

// TODO: should probably move the part for instantiating units into unit.rs
pub fn build_map(
    mut commands: Commands,
    terrain_atlas: Res<TerrainAtlas>,
    unit_atlas: Res<UnitAtlas>,
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

    let units = vec![Unit {
        unit_type: UnitType::Infantry,
        team: Team(0),
        location: Cell { x: 1, y: 1 },
        health: UnitHealth(100),
        selected: Selected(false),
    }];

    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    for (y, row) in game_map.iter().rev().enumerate() {
        for (x, &terrain_index) in row.iter().enumerate() {
            commands.spawn_bundle(SpriteSheetBundle {
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

    for (i, unit) in units.iter().enumerate() {
        let x = unit.location.x;
        let y = unit.location.y;
        commands.spawn_bundle(UnitBundle {
            id: UnitId(i),
            data: unit.clone(),
            sprite: SpriteSheetBundle {
                texture_atlas: unit_atlas.atlas_handle.clone(),
                sprite: TextureAtlasSprite::new(0),
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
