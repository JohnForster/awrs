use std::collections::HashMap;

use bevy::prelude::*;

use super::unit::UnitType;

#[derive(Resource)]
pub struct AtlasResource {
    pub layout: Handle<TextureAtlasLayout>,
    pub texture: Handle<Image>,
}

pub type TerrainAtlas = AtlasResource;

pub type CursorAtlas = AtlasResource;

pub type UnitAtlas = AtlasResource;

pub type HealthAtlas = AtlasResource;

pub type UIAtlas = AtlasResource;

pub type ArrowAtlas = AtlasResource;

#[derive(Resource)]
pub struct UnitAtlases {
    pub atlas_map: HashMap<UnitType, UnitAtlas>,
}
