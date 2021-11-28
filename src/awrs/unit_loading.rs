use bevy::{prelude::*, reflect::TypeUuid};
use serde::*;

use super::load_assets::AssetsLoading;

#[derive(Serialize, Deserialize, Debug, TypeUuid)]
#[uuid = "5386B529-81CC-405A-9600-CB51B83F8CC9"]
pub enum UnitTag {
    Biological,
    Mechanical,
    Light,
    Ground,
    Air,
    Infantry,
}

#[derive(Serialize, Deserialize, Debug, TypeUuid)]
#[uuid = "534753A6-C796-4792-885E-A52C7D7CBF07"]
pub struct Bonus {
    pub tag: UnitTag,
    pub additional_damage: f32,
}

#[derive(Serialize, Deserialize, Debug, TypeUuid)]
#[uuid = "76D0BCF9-21FD-451F-8C1F-D0E600A58D0A"]
pub enum Directness {
    Melee,
    Ranged(f32, f32), // Min, Max
}

#[derive(Serialize, Deserialize, Debug, TypeUuid)]
#[uuid = "21CD8CEB-ED1B-4D20-921D-B775C4E31DBF"]
pub struct Weapon {
    pub id: usize,
    pub name: String,
    pub directness: Directness, // TODO Come up with better name
    pub base_damage: f32,
    pub num_of_attacks: f32,
    pub bonuses: Vec<Bonus>,
    pub applicable: Vec<UnitTag>,
}

#[derive(Serialize, Deserialize, Debug, TypeUuid)]
#[uuid = "67B15859-CC4B-4C35-AB9C-5856628833E4"]
pub struct UnitType {
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

pub struct UnitHandle {
    pub handle: Handle<UnitType>,
}

pub fn load_units(
    server: Res<AssetServer>,
    mut commands: Commands,
    mut loading: ResMut<AssetsLoading>,
) {
    info!("Loading Unit Data");
    let handle = server.load("units/infantry.ron");
    loading.0.push(handle.clone_untyped());
    info!("Unit data loading underway...");
    commands.insert_resource(UnitHandle { handle });
}
