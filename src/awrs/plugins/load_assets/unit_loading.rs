use bevy::prelude::*;
use serde::*;

use super::AssetsLoading;

// TODO Move Into Engine
#[derive(Serialize, Deserialize, Debug, Asset, bevy::reflect::TypePath)]
pub enum UnitTag {
    Biological,
    Mechanical,
    Light,
    Ground,
    Air,
    Infantry,
}

#[derive(Serialize, Deserialize, Debug, Asset, bevy::reflect::TypePath)]
pub struct Bonus {
    pub tag: UnitTag,
    pub additional_damage: f32,
}

#[derive(Serialize, Deserialize, Debug, Asset, bevy::reflect::TypePath)]
pub enum Directness {
    Melee,
    Ranged(f32, f32), // Min, Max
}

#[derive(Serialize, Deserialize, Debug, Asset, bevy::reflect::TypePath)]
pub struct Weapon {
    pub id: usize,
    pub name: String,
    pub directness: Directness, // TODO Come up with better name
    pub base_damage: f32,
    pub num_of_attacks: f32,
    pub bonuses: Vec<Bonus>,
    pub applicable: Vec<UnitTag>,
}

#[derive(Serialize, Deserialize, Debug, Asset, bevy::reflect::TypePath)]
pub struct UnitStats {
    pub id: usize,
    pub name: String,
    pub max_health: f32,
    pub max_ammo: f32,
    pub max_fuel: f32,
    pub tags: Vec<UnitTag>,
    pub base_armour: f32,
    // pub weapon_one: Option<Weapon>,
    // pub weapon_two: Option<Weapon>,
}

pub fn load_units(server: Res<AssetServer>, mut loading: ResMut<AssetsLoading>) {
    info!("Loading Unit Data");
    let handle = server.load("units/infantry.unit.ron");
    loading.0.push(handle.clone());
}
