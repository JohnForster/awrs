use super::units::{UnitTag, UnitTag::*};

pub struct Bonus {
    pub tag: UnitTag,
    pub additional_damage: f32,
}

pub enum Directness {
    Melee,
    Ranged(f32, f32), // Min, Max
}

pub struct Weapon {
    pub id: usize,
    pub name: &'static str,
    pub directness: Directness, // TODO Come up with better name
    pub base_damage: f32,
    pub num_of_attacks: u32,
    pub bonuses: [Option<Bonus>; 4],
    pub applicable: [Option<UnitTag>; 4],
}

pub const ZERGLING_ATTACK: Weapon = Weapon {
    id: 0,
    name: "Zergling Claws",
    directness: Directness::Melee,
    base_damage: 50.0,
    bonuses: [None, None, None, None],
    num_of_attacks: 1,
    applicable: [Some(Ground), None, None, None],
};
