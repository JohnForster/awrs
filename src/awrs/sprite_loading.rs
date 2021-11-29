use bevy::{asset::LoadState, prelude::*};

use super::load_assets::AssetsLoading;

#[derive(Default)]
pub struct TerrainAtlas {
    texture_handle: Handle<Texture>,
    pub atlas_handle: Handle<TextureAtlas>,
}

#[derive(Default)]
pub struct UIAtlas {
    texture_handle: Handle<Texture>,
    pub atlas_handle: Handle<TextureAtlas>,
}

#[derive(Default)]
pub struct UnitAtlas {
    texture_handle: Handle<Texture>,
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

    let mut texture_atlas = TextureAtlas::new_empty(texture_handle.clone(), Vec2::new(143.0, 64.0));

    let cursor_rect = bevy::sprite::Rect {
        min: Vec2::new(44.0, 6.0),
        max: Vec2::new(44.0 + 28.0, 6.0 + 32.0),
    };

    texture_atlas.add_texture(cursor_rect);
    let atlas_handle = texture_atlases.add(texture_atlas);

    commands.insert_resource(UIAtlas {
        texture_handle,
        atlas_handle,
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

    texture_atlas.add_texture(inf_orange_sprite);
    let atlas_handle = texture_atlases.add(texture_atlas);

    commands.insert_resource(UnitAtlas {
        texture_handle,
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
        texture_handle,
        atlas_handle,
    })
}
