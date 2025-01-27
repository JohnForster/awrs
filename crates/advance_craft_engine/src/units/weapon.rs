use serde::{Deserialize, Serialize};

use super::units::{DamageTag, DamageTag::*};

#[derive(PartialEq, Serialize, Deserialize)]
pub struct Bonus {
    pub tag: DamageTag,
    pub additional_damage: f32,
}

#[derive(PartialEq, Serialize, Deserialize)]
pub enum Delivery {
    Melee,
    Ranged(f32, f32), // Min, Max
    Splash(Splash),
}

#[derive(PartialEq, Clone, Serialize, Deserialize)]
pub enum AdditionalEffect {
    Suicide,
}

#[derive(PartialEq, Clone, Copy, Serialize, Deserialize)]
pub struct Splash {
    pub range: (f32, f32),
    pub radius: f32,
    pub friendly: bool,
    // pub _dropoff: f32, // Dropoff per unit range.
}

#[derive(PartialEq, Serialize, Deserialize)]
pub struct Weapon {
    pub _id: usize,
    pub _name: &'static str,
    pub delivery: Delivery,
    pub base_damage: f32,
    pub _num_of_attacks: u32,
    pub bonuses: [Option<Bonus>; 4],
    pub _applicable: [Option<DamageTag>; 4],
    pub additional_effects: [Option<AdditionalEffect>; 4],
}

impl Weapon {
    pub fn has_effect(&self, additional_effect: &AdditionalEffect) -> bool {
        self.additional_effects
            .contains(&Some(additional_effect.clone()))
    }
}

pub const ZERGLING_ATTACK: Weapon = Weapon {
    _id: 1,
    _name: "Zergling Claws",
    delivery: Delivery::Melee,
    base_damage: 10.0,
    bonuses: [None, None, None, None],
    _num_of_attacks: 1,
    _applicable: [Some(Ground), None, None, None],
    additional_effects: [None, None, None, None],
};

pub const BANELING_ATTACK: Weapon = Weapon {
    _id: 2,
    _name: "Acid Boom",
    delivery: Delivery::Splash(Splash {
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
    _num_of_attacks: 1,
    _applicable: [Some(Ground), None, None, None],
    additional_effects: [Some(AdditionalEffect::Suicide), None, None, None],
};

pub const MARINE_ATTACK: Weapon = Weapon {
    _id: 0,
    _name: "Machine Gun",
    delivery: Delivery::Melee,
    base_damage: 9.8,
    bonuses: [None, None, None, None],
    _num_of_attacks: 1,
    _applicable: [Some(Ground), None, None, None],
    additional_effects: [None, None, None, None],
};

pub const ROACH_ATTACK: Weapon = Weapon {
    _id: 3,
    _name: "Acid Saliva",
    delivery: Delivery::Ranged(1.0, 2.0),
    base_damage: 11.2,
    bonuses: [None, None, None, None],
    _num_of_attacks: 1,
    _applicable: [Some(Ground), None, None, None],
    additional_effects: [None, None, None, None],
};

pub const SIEGED_ATTACK: Weapon = Weapon {
    _id: 4,
    _name: "Crucio Shock Cannon",
    delivery: Delivery::Splash(Splash {
        range: (2.0, 4.0),
        radius: 1.5,
        friendly: true,
    }),
    base_damage: 18.9,
    bonuses: [
        Some(Bonus {
            tag: Armoured,
            additional_damage: 14.0,
        }),
        None,
        None,
        None,
    ],
    _num_of_attacks: 1,
    _applicable: [Some(Ground), None, None, None],
    additional_effects: [None, None, None, None],
};
