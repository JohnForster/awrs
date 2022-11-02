use crate::awrs::engine::Tile;

use super::units::{UnitTag, UnitTag::*};

pub struct Bonus {
    pub tag: UnitTag,
    pub additional_damage: f32,
}

pub enum Directness {
    Melee,
    Ranged(f32, f32), // Min, Max
    Splash(Splash),
}

pub enum AdditionalEffect {
    Suicide,
}

pub struct Splash {
    pub range: (f32, f32),
    pub radius: f32,
    pub friendly: bool,
    // pub _dropoff: f32, // Dropoff per unit range.
}

pub struct Weapon {
    pub id: usize,
    pub name: &'static str,
    pub directness: Directness, // TODO Come up with better name
    pub base_damage: f32,
    pub num_of_attacks: u32,
    pub bonuses: [Option<Bonus>; 4],
    pub applicable: [Option<UnitTag>; 4],
    pub additional_effects: [Option<AdditionalEffect>; 4],
}

pub const ZERGLING_ATTACK: Weapon = Weapon {
    id: 1,
    name: "Zergling Claws",
    directness: Directness::Melee,
    base_damage: 10.0,
    bonuses: [None, None, None, None],
    num_of_attacks: 1,
    applicable: [Some(Ground), None, None, None],
    additional_effects: [None, None, None, None],
};

pub const BANELING_ATTACK: Weapon = Weapon {
    id: 2,
    name: "Acid Boom",
    directness: Directness::Splash(Splash {
        range: (0.0, 0.0),
        radius: 1.5,
        friendly: false,
    }),
    base_damage: 16.0,
    bonuses: [
        Some(Bonus {
            tag: Light,
            additional_damage: 19.0,
        }),
        None,
        None,
        None,
    ],
    num_of_attacks: 1,
    applicable: [Some(Ground), None, None, None],
    additional_effects: [Some(AdditionalEffect::Suicide), None, None, None],
};

pub const MARINE_ATTACK: Weapon = Weapon {
    id: 0,
    name: "Machine Gun",
    directness: Directness::Melee,
    base_damage: 9.8,
    bonuses: [None, None, None, None],
    num_of_attacks: 1,
    applicable: [Some(Ground), None, None, None],
    additional_effects: [None, None, None, None],
};

pub const ROACH_ATTACK: Weapon = Weapon {
    id: 3,
    name: "Acid Saliva",
    directness: Directness::Ranged(0.0, 1.0),
    base_damage: 11.2,
    bonuses: [None, None, None, None],
    num_of_attacks: 1,
    applicable: [Some(Ground), None, None, None],
    additional_effects: [None, None, None, None],
};
