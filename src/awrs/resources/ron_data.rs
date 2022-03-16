use bevy::prelude::*;
use bevy::reflect::TypeUuid;

pub struct UnitSpriteData(pub Handle<UnitSpriteDataAsset>);

#[derive(serde::Deserialize, serde::Serialize, TypeUuid, Debug)]
#[uuid = "1df82c01-9c71-4fa8-adc4-78c5822268f8"]
pub struct UnitSpriteDataList(pub Vec<UnitSpriteDataAsset>);

#[derive(serde::Deserialize, serde::Serialize, TypeUuid, Debug)]
#[uuid = "1df82c01-9c71-4fa8-adc4-78c5822268f8"]
pub struct UnitSpriteDataAsset {
    pub unit_id: u32,
    pub top_left: (u32, u32),
    pub dimensions: (u32, u32),
    pub frames: u32,
    pub spacing: u32,
    pub name: String,
}
