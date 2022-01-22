use bevy::prelude::*;

use crate::awrs::resources::atlases::{
    ArrowAtlas, CursorAtlas, HealthAtlas, TerrainAtlas, UIAtlas, UnitAtlas,
};

use super::AssetsLoading;

pub fn load_images(asset_server: Res<AssetServer>, mut loading: ResMut<AssetsLoading>) {
    let paths = [
        "spritesheets/UISprites.png",
        "spritesheets/units.png",
        "spritesheets/unitSprites.png",
    ];

    for &path in paths.iter() {
        let texture_handle: Handle<Texture> = asset_server.load(path);
        loading.0.push(texture_handle.clone_untyped());
    }
}

pub fn create_ui_sprites(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    info!("Loading UI Sprites");
    let texture_handle = asset_server.load("spritesheets/UISprites.png");

    let image_size = Vec2::new(143.0, 64.0);
    let mut cursor_texture_atlas = TextureAtlas::new_empty(texture_handle.clone(), image_size);
    let mut ui_texture_atlas = TextureAtlas::new_empty(texture_handle.clone(), image_size);

    let cursor_rect = bevy::sprite::Rect {
        min: Vec2::new(44.0, 5.0),
        max: Vec2::new(44.0 + 29.0, 6.0 + 32.0),
    };

    let attack_cursor_rect = bevy::sprite::Rect {
        min: Vec2::new(75.0, 5.0),
        max: Vec2::new(75.0 + 29.0, 5.0 + 32.0),
    };

    let movement_overlay_rect = bevy::sprite::Rect {
        min: Vec2::new(7.0, 10.0),
        max: Vec2::new(7.0 + 16.0, 10.0 + 16.0),
    };

    cursor_texture_atlas.add_texture(cursor_rect);
    cursor_texture_atlas.add_texture(attack_cursor_rect);

    let icons_texture_handle: Handle<Texture> = asset_server.load("spritesheets/units.png");
    let icons_image_size = Vec2::new(680.0, 756.0);
    let mut health_texture_atlas =
        TextureAtlas::new_empty(icons_texture_handle.clone(), icons_image_size);

    for n in 0..10 {
        let min = Vec2::new(384.0 + 9.0 * n as f32, 25.0);
        let number_rect = bevy::sprite::Rect {
            min,
            max: min + Vec2::new(8.0, 8.0),
        };
        health_texture_atlas.add_texture(number_rect);
    }

    ui_texture_atlas.add_texture(movement_overlay_rect);

    let cursor_atlas_handle = texture_atlases.add(cursor_texture_atlas);
    let health_atlas_handle = texture_atlases.add(health_texture_atlas);
    let ui_atlas_handle = texture_atlases.add(ui_texture_atlas);

    commands.insert_resource(CursorAtlas {
        atlas_handle: cursor_atlas_handle,
    });
    commands.insert_resource(HealthAtlas {
        atlas_handle: health_atlas_handle,
    });
    commands.insert_resource(UIAtlas {
        atlas_handle: ui_atlas_handle,
    });
}

pub fn create_unit_sprites(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    info!("Loading Unit Sprites");
    let texture_handle = asset_server.load("spritesheets/unitSprites.png");

    let mut texture_atlas =
        TextureAtlas::new_empty(texture_handle.clone(), Vec2::new(349.0, 111.0));

    let inf_orange_sprite = bevy::sprite::Rect {
        min: Vec2::new(23.0, 3.0),
        max: Vec2::new(23.0 + 16.0, 3.0 + 16.0),
    };

    let inf_blue_sprite = bevy::sprite::Rect {
        min: Vec2::new(23.0, 21.0),
        max: Vec2::new(23.0 + 16.0, 21.0 + 16.0),
    };

    let ling_purple_sprite = bevy::sprite::Rect {
        min: Vec2::new(23.0, 93.0),
        max: Vec2::new(23.0 + 16.0, 93.0 + 16.0),
    };

    texture_atlas.add_texture(inf_orange_sprite);
    texture_atlas.add_texture(ling_purple_sprite);
    texture_atlas.add_texture(inf_blue_sprite);
    let atlas_handle = texture_atlases.add(texture_atlas);

    commands.insert_resource(UnitAtlas { atlas_handle })
}

pub fn create_terrain_sprites(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    info!("Loading Terrain Sprites");
    // Terrain Sprites
    let texture_handle = asset_server.load("spritesheets/sprites.png");

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

    commands.insert_resource(TerrainAtlas { atlas_handle })
}

pub fn create_movement_arrow_sprites(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("spritesheets/units.png");

    let mut texture_atlas =
        TextureAtlas::new_empty(texture_handle.clone(), Vec2::new(680.0, 756.0));

    let top_left = Vec2::new(576.0, 139.0);

    for n in 0..24 {
        let min = top_left + Vec2::new(17.0 * (n % 6) as f32, 17.0 * (n / 6) as f32);
        let max = min + Vec2::new(16.0, 16.0);
        let rect = bevy::sprite::Rect { min, max };
        texture_atlas.add_texture(rect);
    }

    let atlas_handle = texture_atlases.add(texture_atlas);
    commands.insert_resource(ArrowAtlas { atlas_handle })
}
