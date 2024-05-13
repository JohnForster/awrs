use super::units::{DamageTag, DamageTag::*};

pub struct Bonus {
    pub tag: DamageTag,
    pub additional_damage: f32,
}

pub enum Delivery {
    Melee,
    Ranged(f32, f32), // Min, Max
    Splash(Splash),
}

#[derive(PartialEq, Clone)]
pub enum AdditionalEffect {
    Suicide,
}

#[derive(Clone, Copy)]
pub struct Splash {
    pub range: (f32, f32),
    pub radius: f32,
    pub friendly: bool,
    // pub _dropoff: f32, // Dropoff per unit range.
}

pub struct Weapon {
    pub id: usize,
    pub name: &'static str,
    pub directness: Delivery, // TODO Come up with better name
    pub base_damage: f32,
    pub num_of_attacks: u32,
    pub bonuses: [Option<Bonus>; 4],
    pub applicable: [Option<DamageTag>; 4],
    pub additional_effects: [Option<AdditionalEffect>; 4],
}

impl Weapon {
    pub fn has_effect(&self, additional_effect: &AdditionalEffect) -> bool {
        self.additional_effects
            .contains(&Some(additional_effect.clone()))

        // self.additional_effects.iter().any(|maybe_effect| {
        //     if let Some(effect) = maybe_effect {
        //         effect == additional_effect
        //     } else {
        //         false
        //     }
        // })
    }
}

pub const ZERGLING_ATTACK: Weapon = Weapon {
    id: 1,
    name: "Zergling Claws",
    directness: Delivery::Melee,
    base_damage: 10.0,
    bonuses: [None, None, None, None],
    num_of_attacks: 1,
    applicable: [Some(Ground), None, None, None],
    additional_effects: [None, None, None, None],
};

pub const BANELING_ATTACK: Weapon = Weapon {
    id: 2,
    name: "Acid Boom",
    directness: Delivery::Splash(Splash {
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
    directness: Delivery::Melee,
    base_damage: 9.8,
    bonuses: [None, None, None, None],
    num_of_attacks: 1,
    applicable: [Some(Ground), None, None, None],
    additional_effects: [None, None, None, None],
};

pub const ROACH_ATTACK: Weapon = Weapon {
    id: 3,
    name: "Acid Saliva",
    directness: Delivery::Ranged(1.0, 2.0),
    base_damage: 11.2,
    bonuses: [None, None, None, None],
    num_of_attacks: 1,
    applicable: [Some(Ground), None, None, None],
    additional_effects: [None, None, None, None],
};

pub const SIEGED_ATTACK: Weapon = Weapon {
    id: 4,
    name: "Crucio Shock Cannon",
    directness: Delivery::Splash(Splash {
        range: (1.0, 3.0),
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
    num_of_attacks: 1,
    applicable: [Some(Ground), None, None, None],
    additional_effects: [None, None, None, None],
};
