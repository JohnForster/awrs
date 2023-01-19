use super::weapon::*;

#[derive(Clone, Copy, Debug)]
pub enum UnitType {
    Zergling,
    Roach,
    Marine,
    Baneling,
}

#[derive(PartialEq)]
pub enum UnitTag {
    Biological,
    _Mechanical,
    Light,
    Armoured,
    Ground,
    _Air,
    _Infantry,
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
    max_health: 35.0,
    max_ammo: -1.0,
    max_fuel: -1.0,
    tags: [Some(Ground), Some(Light), Some(Biological), None],
    base_armour: 0.0,
    weapon_one: Some(ZERGLING_ATTACK),
    weapon_two: None,
};

pub const BANELING: UnitStats = UnitStats {
    id: 2,
    race: Race::Zerg,
    name: "Baneling",
    max_health: 30.0,
    max_ammo: -1.0,
    max_fuel: -1.0,
    tags: [Some(Ground), Some(Light), Some(Biological), None],
    base_armour: 0.0,
    weapon_one: Some(BANELING_ATTACK),
    weapon_two: None,
};

pub const MARINE: UnitStats = UnitStats {
    id: 0,
    race: Race::Terran,
    name: "Marine",
    max_health: 55.0,
    max_ammo: -1.0,
    max_fuel: -1.0,
    tags: [Some(Ground), Some(Light), Some(Biological), None],
    base_armour: 0.0,
    weapon_one: Some(MARINE_ATTACK),
    weapon_two: None,
};

pub const ROACH: UnitStats = UnitStats {
    id: 4,
    race: Race::Zerg,
    name: "Roach",
    max_health: 145.0,
    max_ammo: -1.0,
    max_fuel: -1.0,
    tags: [Some(Ground), Some(Armoured), Some(Biological), None],
    base_armour: 1.0,
    weapon_one: Some(ROACH_ATTACK),
    weapon_two: None,
};
//    Unit     Range     Speed
// Marine      5 => 2    3.2 => 3
// Marauder    5 => 2    3.2 => 3
// Hellion     5 => 2    6.0 => 5
// Zergling    M => M    6.6 => 6
// Baneling    M => M    4.1 => 4
// Roach       5 => 2    3.2 => 3
// Zealot      M => M    3.2 => 3
// Stalker     6 => 2    4.1 => 4
// Immortal    6 => 2    3.2 => 3

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
