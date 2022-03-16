use bevy::prelude::*;

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
    pub handles: Vec<Handle<TextureAtlas>>,
}
