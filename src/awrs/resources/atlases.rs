use std::collections::HashMap;

use bevy::prelude::*;

use super::unit::{StructureType, UnitType};

#[derive(Resource)]
pub struct TerrainAtlas {
    pub layout: Handle<TextureAtlasLayout>,
    pub texture: Handle<Image>,
}

#[derive(Resource)]
pub struct CreepAtlas {
    pub layout: Handle<TextureAtlasLayout>,
    pub texture: Handle<Image>,
}

#[derive(Resource)]
pub struct CursorAtlas {
    pub layout: Handle<TextureAtlasLayout>,
    pub texture: Handle<Image>,
}

#[derive(Resource)]
pub struct UnitAtlas {
    pub layout: Handle<TextureAtlasLayout>,
    pub texture: Handle<Image>,
}

#[derive(Resource)]
pub struct HealthAtlas {
    pub layout: Handle<TextureAtlasLayout>,
    pub texture: Handle<Image>,
}

#[derive(Resource)]
pub struct UIAtlas {
    pub _layout: Handle<TextureAtlasLayout>,
    pub _texture: Handle<Image>,
}

#[derive(Resource)]
pub struct ArrowAtlas {
    pub layout: Handle<TextureAtlasLayout>,
    pub texture: Handle<Image>,
}

#[derive(Resource)]
pub struct StructureAtlas {
    pub layout: Handle<TextureAtlasLayout>,
    pub texture: Handle<Image>,
}

#[derive(Resource)]
pub struct UnitAtlases {
    pub atlas_map: HashMap<UnitType, UnitAtlas>,
}

#[derive(Resource)]
pub struct StructureAtlases {
    pub atlas_map: HashMap<StructureType, StructureAtlas>,
}
