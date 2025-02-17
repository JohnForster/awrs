use std::collections::HashMap;

use bevy::prelude::*;

use crate::awrs::resources::{
    atlases::{
        ArrowAtlas, CreepAtlas, CursorAtlas, HealthAtlas, StructureAtlas, StructureAtlases,
        TerrainAtlas, UIAtlas, UnitAtlas, UnitAtlases,
    },
    unit::StructureType,
    unit::UnitType,
};

use super::AssetsLoading;

pub fn load_images(asset_server: Res<AssetServer>, mut loading: ResMut<AssetsLoading>) {
    let paths = [
        "spritesheets/UISprites.png",
        "spritesheets/units.png",
        "spritesheets/unitSprites.png",
        "spritesheets/infantry_idle.png",
        "spritesheets/zergling_idle.png",
        "spritesheets/baneling_idle.png",
        "spritesheets/roach_idle.png",
    ];

    for &path in paths.iter() {
        let texture_handle: Handle<Image> = asset_server.load(path);
        loading.0.push(UntypedHandle::from(texture_handle));
    }
}

pub fn create_ui_sprites(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
    info!("Loading UI Sprites");
    let ui_texture = asset_server.load("spritesheets/UISprites.png");
    let image_size = UVec2::new(143, 64);

    // Set up Cursor
    let mut cursor_layout = TextureAtlasLayout::new_empty(image_size);

    let cursor_rect = bevy::math::URect {
        min: UVec2::new(44, 5),
        max: UVec2::new(44 + 29, 6 + 32),
    };

    let attack_cursor_rect = bevy::math::URect {
        min: UVec2::new(75, 5),
        max: UVec2::new(75 + 29, 5 + 32),
    };

    // 9, 58, 57x57
    let splash_cursor_rect = bevy::math::URect {
        min: UVec2::new(8, 58),
        max: UVec2::new(8 + 57, 58 + 57),
    };

    cursor_layout.add_texture(cursor_rect);
    cursor_layout.add_texture(attack_cursor_rect);
    cursor_layout.add_texture(splash_cursor_rect);

    // Set up other UI sprites
    let mut ui_layout = TextureAtlasLayout::new_empty(image_size);

    let movement_overlay_rect = bevy::math::URect {
        min: UVec2::new(7, 10),
        max: UVec2::new(7 + 16, 10 + 16),
    };

    ui_layout.add_texture(movement_overlay_rect);

    let icons_texture: Handle<Image> = asset_server.load("spritesheets/units.png");
    let icons_image_size = UVec2::new(6, 756);
    let mut health_layout = TextureAtlasLayout::new_empty(icons_image_size);

    for n in 0..10 {
        let min = UVec2::new(384 + 9 * n, 25);
        let number_rect = bevy::math::URect {
            min,
            max: min + UVec2::new(8, 8),
        };
        health_layout.add_texture(number_rect);
    }

    let cursor_layout_handle = texture_atlases.add(cursor_layout);
    let health_layout_handle = texture_atlases.add(health_layout);
    let ui_layout_handle = texture_atlases.add(ui_layout);

    commands.insert_resource(CursorAtlas {
        texture: ui_texture.clone(),
        layout: cursor_layout_handle,
    });
    commands.insert_resource(HealthAtlas {
        texture: icons_texture,
        layout: health_layout_handle,
    });
    commands.insert_resource(UIAtlas {
        _texture: ui_texture.clone(),
        _layout: ui_layout_handle,
    });
}

pub fn create_idle_sprites(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
    let mut unit_atlas_handle_map: HashMap<UnitType, UnitAtlas> = HashMap::new();

    let units = [
        (UnitType::Infantry, "spritesheets/marine_idle.png"),
        (UnitType::Zergling, "spritesheets/zergling_idle.png"),
        (UnitType::Baneling, "spritesheets/baneling_idle.png"),
        (UnitType::Roach, "spritesheets/roach_idle.png"),
        (UnitType::SiegeTank, "spritesheets/tank_idle.png"),
    ];

    for (unit_type, idle_path) in units {
        let image_handle = asset_server.load(idle_path);
        let layout =
            TextureAtlasLayout::from_grid(UVec2::new(16, 16), 4, 1, Some(UVec2::new(1, 0)), None);
        let layout_handle = atlases.add(layout);
        unit_atlas_handle_map.insert(
            unit_type,
            UnitAtlas {
                texture: image_handle,
                layout: layout_handle,
            },
        );
    }

    commands.insert_resource(UnitAtlases {
        atlas_map: unit_atlas_handle_map,
    });
}

pub fn create_structure_sprites(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
    let mut structure_atlas_handle_map: HashMap<StructureType, StructureAtlas> = HashMap::new();

    let structures = [
        (
            StructureType::CommandCentre,
            "spritesheets/command_centre.png",
        ),
        (StructureType::Hatchery, "spritesheets/hatchery.png"),
    ];

    for (structure_type, spritesheet_path) in structures {
        let image_handle = asset_server.load(spritesheet_path);
        let layout =
            TextureAtlasLayout::from_grid(UVec2::new(48, 48), 4, 1, Some(UVec2::new(1, 0)), None);
        let layout_handle = atlases.add(layout);
        structure_atlas_handle_map.insert(
            structure_type,
            StructureAtlas {
                texture: image_handle,
                layout: layout_handle,
            },
        );
    }

    commands.insert_resource(StructureAtlases {
        atlas_map: structure_atlas_handle_map,
    });
}

pub fn create_terrain_sprites(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
    info!("Loading Terrain Sprites");
    // Terrain Sprites
    let texture_handle = asset_server.load("spritesheets/sprites.png");

    let mut layout = TextureAtlasLayout::new_empty(UVec2::new(1215, 1744));

    let grass_rect = bevy::math::URect {
        min: UVec2::new(217, 1567),
        max: UVec2::new(217 + 16, 1567 + 16),
    };
    let sea_rect = bevy::math::URect {
        min: UVec2::new(3, 1567),
        max: UVec2::new(3 + 16, 1567 + 16),
    };

    layout.add_texture(sea_rect);
    layout.add_texture(grass_rect);

    let layout_handle = atlases.add(layout);

    commands.insert_resource(TerrainAtlas {
        texture: texture_handle,
        layout: layout_handle,
    })
}

pub fn create_creep_sprites(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
    info!("Loading Terrain Sprites");
    // Terrain Sprites
    let texture_handle = asset_server.load("spritesheets/creep.png");

    let mut layout = TextureAtlasLayout::new_empty(UVec2::new(96, 48));
    const SPRITE_SIZE: u32 = 16;

    info!("Loading creep sprites");
    for y in 0..3 {
        for x in 0..3 {
            // let y = 2 - y;
            let min_x = x * SPRITE_SIZE;
            let min_y = y * SPRITE_SIZE;
            let max_x = min_x + SPRITE_SIZE;
            let max_y = min_y + SPRITE_SIZE;
            info!(
                "{}-{} min:({} {}) max:({} {})",
                x, y, min_x, min_y, max_x, max_y
            );
            let rect = bevy::math::URect {
                min: UVec2::new(min_x, min_y),
                max: UVec2::new(max_x, max_y),
            };

            layout.add_texture(rect);
        }
    }

    let layout_handle = atlases.add(layout);

    commands.insert_resource(CreepAtlas {
        texture: texture_handle,
        layout: layout_handle,
    })
}

pub fn create_movement_arrow_sprites(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture_handle = asset_server.load("spritesheets/units.png");

    let mut layout = TextureAtlasLayout::new_empty(UVec2::new(6, 756));

    let top_left = UVec2::new(576, 139);

    for n in 0..24 {
        let min = top_left + UVec2::new(17 * (n % 6), 17 * (n / 6));
        let max = min + UVec2::new(16, 16);
        let rect = bevy::math::URect { min, max };
        layout.add_texture(rect);
    }

    let layout_handle = texture_atlases.add(layout);
    commands.insert_resource(ArrowAtlas {
        texture: texture_handle,
        layout: layout_handle,
    })
}
