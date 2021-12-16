use bevy::prelude::*;

use super::load_assets::AssetsLoading;

#[derive(Default)]
pub struct TerrainAtlas {
    _texture_handle: Handle<Texture>,
    pub atlas_handle: Handle<TextureAtlas>,
}

#[derive(Default)]
pub struct UIAtlas {
    _texture_handle: Handle<Texture>,
    pub atlas_handle: Handle<TextureAtlas>,
}

#[derive(Default)]
pub struct UnitAtlas {
    _texture_handle: Handle<Texture>,
    pub atlas_handle: Handle<TextureAtlas>,
}

pub struct HealthAtlas {
    pub atlas_handle: Handle<TextureAtlas>,
}

pub fn load_ui_sprites(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut loading: ResMut<AssetsLoading>,
) {
    info!("Loading UI Sprites");
    let texture_handle = asset_server.load("spritesheets/UISprites.png");
    loading.0.push(texture_handle.clone_untyped());
    info!("UI sprite loading underway...");

    let image_size = Vec2::new(143.0, 64.0);
    let mut cursor_texture_atlas = TextureAtlas::new_empty(texture_handle.clone(), image_size);
    let mut health_texture_atlas = TextureAtlas::new_empty(texture_handle.clone(), image_size);

    let cursor_rect = bevy::sprite::Rect {
        min: Vec2::new(44.0, 5.0),
        max: Vec2::new(44.0 + 29.0, 6.0 + 32.0),
    };

    let attack_cursor_rect = bevy::sprite::Rect {
        min: Vec2::new(75.0, 5.0),
        max: Vec2::new(75.0 + 29.0, 5.0 + 32.0),
    };

    cursor_texture_atlas.add_texture(cursor_rect);
    cursor_texture_atlas.add_texture(attack_cursor_rect);

    for n in 0..10 {
        let min = Vec2::new(42.0 + 9.0 * n as f32, 41.0);
        let number_rect = bevy::sprite::Rect {
            min,
            max: min + Vec2::new(8.0, 12.0),
        };
        health_texture_atlas.add_texture(number_rect);
    }
    let cursor_atlas_handle = texture_atlases.add(cursor_texture_atlas);
    let health_atlas_handle = texture_atlases.add(health_texture_atlas);

    commands.insert_resource(UIAtlas {
        _texture_handle: texture_handle,
        atlas_handle: cursor_atlas_handle,
    });
    commands.insert_resource(HealthAtlas {
        atlas_handle: health_atlas_handle,
    })
}

pub fn load_unit_sprites(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut loading: ResMut<AssetsLoading>,
) {
    info!("Loading Unit Sprites");
    let texture_handle = asset_server.load("spritesheets/unitSprites.png");
    loading.0.push(texture_handle.clone_untyped());
    info!("Unit sprite loading underway...");
    let mut texture_atlas = TextureAtlas::new_empty(texture_handle.clone(), Vec2::new(349.0, 93.0));

    let inf_orange_sprite = bevy::sprite::Rect {
        min: Vec2::new(24.0, 4.0),
        max: Vec2::new(24.0 + 14.0, 4.0 + 16.0),
    };

    let inf_blue_sprite = bevy::sprite::Rect {
        min: Vec2::new(24.0, 22.0),
        max: Vec2::new(24.0 + 14.0, 22.0 + 16.0),
    };

    texture_atlas.add_texture(inf_orange_sprite);
    texture_atlas.add_texture(inf_blue_sprite);
    let atlas_handle = texture_atlases.add(texture_atlas);

    commands.insert_resource(UnitAtlas {
        _texture_handle: texture_handle,
        atlas_handle,
    })
}

pub fn load_terrain_sprites(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut loading: ResMut<AssetsLoading>,
) {
    info!("Loading Terrain Sprites");
    // Terrain Sprites
    let texture_handle = asset_server.load("spritesheets/sprites.png");
    loading.0.push(texture_handle.clone_untyped());
    info!("Terrain sprite loading underway...");
    let mut texture_atlas =
        TextureAtlas::new_empty(texture_handle.clone(), Vec2::new(1215.0, 1744.0));

    let grass_rect = bevy::sprite::Rect {
        min: Vec2::new(217.0, 1567.0),
        max: Vec2::new(217.0 + 16.0, 1567.0 + 16.0),
    };
    let sea_rect = bevy::sprite::Rect {
        min: Vec2::new(340.0, 1567.0),
        max: Vec2::new(340.0 + 16.0, 1567.0 + 16.0),
    };

    texture_atlas.add_texture(sea_rect);
    texture_atlas.add_texture(grass_rect);

    let atlas_handle = texture_atlases.add(texture_atlas);

    commands.insert_resource(TerrainAtlas {
        _texture_handle: texture_handle,
        atlas_handle,
    })
}
