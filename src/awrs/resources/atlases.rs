use std::collections::HashMap;

use bevy::prelude::*;

use super::unit::UnitType;

#[derive(Default, Resource)]
pub struct TerrainAtlas {
    pub atlas_handle: Handle<TextureAtlas>,
}

#[derive(Default, Resource)]
pub struct CursorAtlas {
    pub atlas_handle: Handle<TextureAtlas>,
}

#[derive(Default, Resource)]
pub struct UnitAtlas {
    pub atlas_handle: Handle<TextureAtlas>,
}

#[derive(Resource)]
pub struct HealthAtlas {
    pub atlas_handle: Handle<TextureAtlas>,
}

#[derive(Resource)]
pub struct UIAtlas {
    pub atlas_handle: Handle<TextureAtlas>,
}

#[derive(Resource)]
pub struct ArrowAtlas {
    pub atlas_handle: Handle<TextureAtlas>,
}

#[derive(Resource)]
pub struct UnitAtlases {
    pub handle_map: HashMap<UnitType, Handle<TextureAtlas>>,
}
