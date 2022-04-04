use super::weapon::*;

#[derive(Clone, Copy, Debug)]
pub enum UnitType {
    Zergling,
    Roach,
    Marine,
    Baneling,
}

pub enum UnitTag {
    Biological,
    Mechanical,
    Light,
    Ground,
    Air,
    Infantry,
}

pub struct UnitStats {
    pub id: usize,
    pub name: &'static str,
    pub race: Race,
    pub max_health: f32,
    pub max_ammo: f32,
    pub max_fuel: f32,
    pub tags: [Option<UnitTag>; 4],
    pub base_armour: f32,
    pub weapon_one: Option<Weapon>,
    pub weapon_two: Option<Weapon>,
}

pub enum Race {
    Zerg,
    Terran,
    _Protoss,
}

impl UnitType {
    pub const fn value(&self) -> UnitStats {
        use UnitType::*;
        match *self {
            Zergling => ZERGLING,
            Baneling => BANELING,
            Roach => ROACH,
            Marine => MARINE,
        }
    }
}

use UnitTag::*;

pub const ZERGLING: UnitStats = UnitStats {
    id: 1,
    race: Race::Zerg,
    name: "Zergling",
    max_health: 10.0,
    max_ammo: -1.0,
    max_fuel: -1.0,
    tags: [Some(Ground), Some(Light), Some(Biological), None],
    base_armour: 0.0,
    weapon_one: Some(ZERGLING_ATTACK),
    weapon_two: None,
};
pub const ROACH: UnitStats = ZERGLING;
pub const MARINE: UnitStats = ZERGLING;
pub const BANELING: UnitStats = ZERGLING;

// pub const ROACH: UnitStats = UnitStats {
//     id: 1,
//     race: Zerg,
//     name: "Roach",
//     max_health: 10.0,
//     max_ammo: -1.0,
//     max_fuel: -1.0,
//     tags: [Ground, Armoured, Biological],
//     base_armour: 0.0,
//     primary_weapon: ZERGLING_ATTACK,
// };

// pub const MARINE: UnitStats = UnitStats {
//     id: 1,
//     race: Terran,
//     name: "Zergling",
//     max_health: 10.0,
//     max_ammo: -1.0,
//     max_fuel: -1.0,
//     tags: [Ground, Light, Biological],
//     base_armour: 0.0,
//     primary_weapon: ZERGLING_ATTACK,
// };

// pub const BANELING: UnitStats = UnitStats {
//     id: 1,
//     race: Zerg,
//     name: "Zergling",
//     max_health: 10.0,
//     max_ammo: -1.0,
//     max_fuel: -1.0,
//     tags: [Ground, Light, Biological],
//     base_armour: 0.0,
//     primary_weapon: ZERGLING_ATTACK,
// };
