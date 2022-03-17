use std::collections::HashMap;

use bevy::prelude::*;

use super::unit::UnitType;

#[derive(Default)]
pub struct TerrainAtlas {
    pub atlas_handle: Handle<TextureAtlas>,
}

#[derive(Default)]
pub struct CursorAtlas {
    pub atlas_handle: Handle<TextureAtlas>,
}

#[derive(Default)]
pub struct UnitAtlas {
    pub atlas_handle: Handle<TextureAtlas>,
}

pub struct HealthAtlas {
    pub atlas_handle: Handle<TextureAtlas>,
}

pub struct UIAtlas {
    pub atlas_handle: Handle<TextureAtlas>,
}

pub struct ArrowAtlas {
    pub atlas_handle: Handle<TextureAtlas>,
}

pub struct UnitAtlases {
    pub handle_map: HashMap<UnitType, Handle<TextureAtlas>>,
}
