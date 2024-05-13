use std::collections::HashMap;

use super::{
    structures::structures::*,
    units::{units::*, weapon::Weapon},
    weapon::{AdditionalEffect, Delivery},
};
use bevy::{ecs::system::Resource, prelude::info};

#[derive(Debug, Clone, Copy)]
pub struct Tile {
    pub x: u32,
    pub y: u32,
}

impl PartialEq for Tile {
    fn eq(&self, other: &Tile) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Tile {
    pub fn distance_to(&self, other: &Tile) -> f32 {
        let dx = self.x as f32 - other.x as f32;
        let dy = self.y as f32 - other.y as f32;
        return f32::sqrt(dx.powi(2) + dy.powi(2));
    }
}

pub type UnitId = u32;
pub type UnitHp = f32;
pub type Team = u32;

#[derive(Debug)]
pub struct Unit {
    pub id: UnitId,
    pub unit_type: UnitType,
    pub position: Tile,
    pub health: UnitHp,
    pub team: Team,
    pub has_moved: bool,
    pub has_attacked: bool,
}

pub type StructureId = u32;
pub type StructureHp = f32;

#[derive(Debug)]
pub struct Structure {
    pub id: StructureId,
    pub structure_type: StructureType,
    pub position: Tile,
    pub health: StructureHp,
    pub team: Team,
}

#[derive(Debug)]
pub enum TerrainType {
    Grass,
    Water,
}

pub type ScenarioMap = Vec<Vec<TerrainType>>;

trait ScenarioMapMethods {
    fn width(&self) -> u32;

    fn height(&self) -> u32;
}

impl ScenarioMapMethods for ScenarioMap {
    fn width(&self) -> u32 {
        self[0].len() as u32
    }

    fn height(&self) -> u32 {
        self.len() as u32
    }
}

pub trait Contains<T> {
    fn contains(&self, x: &T, y: &T) -> bool;
}

impl Contains<f32> for ScenarioState {
    fn contains(&self, x: &f32, y: &f32) -> bool {
        let width = self.map.width() as f32;
        let height = self.map.height() as f32;
        return *x >= 0.0 && *x < width && *y >= 0.0 && *y < height;
    }
}

impl Contains<i32> for ScenarioState {
    fn contains(&self, x: &i32, y: &i32) -> bool {
        let width = self.map.width() as i32;
        let height = self.map.height() as i32;
        return (0..width).contains(x) && (0..height).contains(y);
    }
}

pub enum _Moveable {
    Through,
    Stop,
    Blocked,
}

pub enum UnitAction {
    Move,
    Attack,
    _Capture,
    _Join,
    _SelfDestruct,
    _Resupply,
}

#[derive(Debug)]
pub enum Command {
    Move {
        unit_id: UnitId,
        tiles: Vec<Tile>,
    },
    Attack {
        attacker_id: UnitId,
        defender_id: UnitId,
    },
    AttackGround {
        attacker_id: UnitId,
        tile: Tile,
    },
    EndTurn,
}

pub type TeamID = u32;

#[derive(Debug, Resource)]
pub struct ScenarioState {
    pub map: ScenarioMap,
    pub units: Vec<Unit>,
    pub structures: Vec<Structure>,
    pub active_team: TeamID,
    pub teams: Vec<TeamID>,
    pub creep: Creep,
}

pub type CreepMap = HashMap<TeamID, Vec<Vec<bool>>>;

#[derive(Debug)]
pub struct Creep(pub CreepMap);

impl Creep {
    pub fn empty(number_of_teams: u32, map: &ScenarioMap) -> Self {
        let mut creep_map: CreepMap = HashMap::new();
        let h = map.len();
        let w = map[0].len();
        for i in 0..number_of_teams {
            creep_map.insert(i, vec![vec![false; w]; h]);
        }

        return Self(creep_map);
    }

    pub fn set(&mut self, id: TeamID, map: Vec<Vec<bool>>) {
        self.0.insert(id, map);
    }
}

#[derive(Debug)]
pub enum CommandStatus {
    Ok,
    Partial,
    Err(CommandErr),
}

#[derive(Debug)]
pub enum CommandErr {
    AlreadyMoved,
    AlreadyAttacked,
    NotImplemented,
    OutOfRange,
    UnknownErr,
}

#[derive(Debug)]
pub enum CommandResult {
    Move {
        status: CommandStatus,
        tiles: Vec<Tile>,
    },
    AttackGround {
        status: CommandStatus,
        unit_hp_changes: Vec<(UnitId, UnitHp)>,
    },
    Attack {
        status: CommandStatus,
        unit_hp_changes: Vec<(UnitId, UnitHp)>,
    },
    EndTurn {
        status: CommandStatus,
        new_active_team: Team,
    },
}

// Mutating
impl ScenarioState {
    pub fn execute(&mut self, command: Command) -> CommandResult {
        match command {
            Command::Move { unit_id, tiles } => self.unit_move(unit_id, tiles),
            Command::Attack {
                attacker_id,
                defender_id,
            } => self.attack(attacker_id, defender_id),
            Command::AttackGround { attacker_id, tile } => self.attack_ground(attacker_id, tile),
            Command::EndTurn => self.end_turn(),
        }
    }

    fn unit_move(&mut self, id: UnitId, tiles: Vec<Tile>) -> CommandResult {
        let mut units_iterator = self.units.iter_mut();

        let unit = units_iterator
            .find(|u| u.id == id)
            .expect(format!("No unit found with id {}", id).as_str());

        if unit.has_moved {
            return CommandResult::Move {
                status: CommandStatus::Err(CommandErr::AlreadyMoved),
                tiles: vec![unit.position],
            };
        }

        let mut successful_moves: Vec<Tile> = vec![];
        let mut pending_moves: Vec<Tile> = vec![];
        let mut status = CommandStatus::Err(CommandErr::UnknownErr);

        for Tile { x, y } in tiles {
            // TODO - Check valid
            //   Adjacent
            //   Terrain/Fuel
            //   Hasn't already moved

            // Check that each tile is free
            let maybe_blocking_unit =
                units_iterator.find(|u| u.position.x == x && u.position.y == y);

            match maybe_blocking_unit {
                None => {
                    successful_moves.append(&mut pending_moves);
                }
                Some(other_unit) => {
                    status = CommandStatus::Partial;
                    if other_unit.team == unit.team {
                        pending_moves.push(Tile { x, y });
                        continue;
                    } else {
                        break;
                    }
                }
            }

            // Move the unit to there
            unit.position.x = x;
            unit.position.y = y;

            // Reduce movement according to terrain
            // Fuel etc.

            successful_moves.push(Tile { x, y });
            status = CommandStatus::Ok;
        }

        // Set has_moved true
        unit.has_moved = true;

        return CommandResult::Move {
            status,
            tiles: successful_moves,
        };
    }

    fn attack(&mut self, attacker_id: UnitId, defender_id: UnitId) -> CommandResult {
        // Validate attack
        //   Range
        //   Ammo
        //   Turn etc.
        //   Weapon type

        let (attacker, defender) = self.get_two_units(attacker_id, defender_id).unwrap();

        // Attacker is able to attack
        if attacker.has_attacked {
            return CommandResult::Attack {
                status: CommandStatus::Err(CommandErr::AlreadyAttacked),
                unit_hp_changes: vec![
                    (attacker.id, attacker.health),
                    (defender.id, defender.health),
                ],
            };
        }

        // Choose Weapon
        let in_range = check_range(attacker, defender);

        if !in_range {
            return CommandResult::Attack {
                status: CommandStatus::Err(CommandErr::OutOfRange),
                unit_hp_changes: vec![
                    (attacker.id, attacker.health),
                    (defender.id, defender.health),
                ],
            };
        }
        // Check Ammo

        // Calculate damage
        let (attacker_damage, defender_damage) = self.calculate_damage(attacker.id, defender.id);

        let (attacker, defender) = self.get_two_units_mut(attacker_id, defender_id).unwrap();

        attacker.health -= attacker_damage;
        defender.health -= defender_damage;

        attacker.has_attacked = true;

        return CommandResult::Attack {
            status: CommandStatus::Ok,
            unit_hp_changes: vec![
                (attacker.id, attacker.health),
                (defender.id, defender.health),
            ],
        };
    }

    fn attack_ground(&mut self, attacker_id: UnitId, tile: Tile) -> CommandResult {
        let attacker = self.get_unit(attacker_id).unwrap();
        let weapon = attacker.unit_type.value().weapon_one.unwrap();
        match weapon.directness {
            Delivery::Splash(splash) => {
                let tile_in_range = check_range_to_tile(attacker, &tile);
                info!("{:?}", tile_in_range);
                if !tile_in_range {
                    return CommandResult::AttackGround {
                        status: CommandStatus::Err(CommandErr::OutOfRange),
                        unit_hp_changes: vec![],
                    };
                }

                let units_in_range = self.get_units_within_radius(tile, splash.radius);

                let mut damaged_units = HashMap::new();
                for unit in units_in_range.iter() {
                    let same_team = unit.team == attacker.team;
                    if same_team && !splash.friendly {
                        continue;
                    }

                    let damage = self.calculate_damage(attacker_id, unit.id).1;
                    damaged_units.insert(unit.id, damage);
                }

                let mut unit_hp_changes = vec![];
                for unit in self.units.iter_mut() {
                    match damaged_units.get(&unit.id) {
                        Some(damage) => {
                            unit.health -= damage;
                            unit_hp_changes.push((unit.id, unit.health));
                        }
                        None => continue,
                    }
                }

                let is_suicide = weapon.has_effect(&AdditionalEffect::Suicide);

                if is_suicide {
                    unit_hp_changes.push((attacker_id, 0.0));
                }

                CommandResult::AttackGround {
                    status: CommandStatus::Ok,
                    unit_hp_changes,
                }
            }
            _ => CommandResult::AttackGround {
                status: CommandStatus::Err(CommandErr::NotImplemented),
                unit_hp_changes: vec![],
            },
        }
    }

    fn end_turn(&mut self) -> CommandResult {
        info!("Ending turn");
        let new_active_team = (self.active_team + 1) % (self.teams.len() as u32);
        self.active_team = new_active_team;
        for unit in self.units.iter_mut() {
            unit.has_attacked = false;
            unit.has_moved = false;
        }
        return CommandResult::EndTurn {
            status: CommandStatus::Ok,
            new_active_team,
        };
    }

    pub fn get_two_units_mut(
        &mut self,
        attacker_id: UnitId,
        defender_id: UnitId,
    ) -> Option<(&mut Unit, &mut Unit)> {
        let mut maybe_attacker: Option<&mut Unit> = None;
        let mut maybe_defender: Option<&mut Unit> = None;
        for unit in self.units.iter_mut() {
            if unit.id == attacker_id {
                maybe_attacker = Some(unit);
            } else if unit.id == defender_id {
                maybe_defender = Some(unit);
            }
        }
        let attacker = maybe_attacker.expect("No attacker found");
        let defender = maybe_defender.expect("No defender found");

        return Some((attacker, defender));
    }

    pub fn get_two_units(
        &self,
        attacker_id: UnitId,
        defender_id: UnitId,
    ) -> Option<(&Unit, &Unit)> {
        let mut maybe_attacker: Option<&Unit> = None;
        let mut maybe_defender: Option<&Unit> = None;
        for unit in self.units.iter() {
            if unit.id == attacker_id {
                maybe_attacker = Some(unit);
            } else if unit.id == defender_id {
                maybe_defender = Some(unit);
            }
        }
        let attacker = maybe_attacker.expect("No attacker found");
        let defender = maybe_defender.expect("No defender found");

        return Some((attacker, defender));
    }
}

fn check_range(attacker: &Unit, defender: &Unit) -> bool {
    let attacker_weapon = attacker
        .unit_type
        .value()
        .weapon_one
        .expect("No weapon found");

    let (min, max) = match attacker_weapon.directness {
        Delivery::Melee => (1.0, 1.0),
        Delivery::Ranged(min, max) => (min, max),
        _ => (1.0, 1.0),
    };
    let distance_between_units = attacker.position.distance_to(&defender.position);
    let in_range = distance_between_units >= min && distance_between_units <= max;
    in_range
}

fn check_range_to_tile(attacker: &Unit, tile: &Tile) -> bool {
    let attacker_weapon = attacker
        .unit_type
        .value()
        .weapon_one
        .expect("No weapon found");

    let (min, max) = match attacker_weapon.directness {
        Delivery::Melee => (1.0, 1.0),
        Delivery::Ranged(min, max) => (min, max),
        Delivery::Splash(splash) => splash.range,
    };
    let distance_to_tile = attacker.position.distance_to(tile);
    let in_range = distance_to_tile >= min && distance_to_tile <= max;
    in_range
}

// Non mutating
impl ScenarioState {
    pub fn get_unit(&self, unit_id: UnitId) -> Option<&Unit> {
        self.units.iter().find(|u| u.id == unit_id)
    }

    pub fn get_moveable_tiles(&self, unit_id: UnitId) -> Vec<Tile> {
        let mut moveable_tiles = vec![];
        let unit = self.get_unit(unit_id).expect("No unit found!");
        let range = self.get_movement_range(&unit_id) as i32;

        // Add all tiles within a given movement range
        for dx in -range..=range {
            let remaining_range = range - dx.abs();
            for dy in -remaining_range..=remaining_range {
                let new_x = unit.position.x as i32 + dx;
                let new_y = unit.position.y as i32 + dy;

                if self.is_tile_moveable(unit_id, new_x, new_y) {
                    moveable_tiles.push(Tile {
                        x: new_x as u32,
                        y: new_y as u32,
                    })
                }
            }
        }

        return moveable_tiles;
    }

    pub fn calculate_damage(&self, attacker_id: UnitId, defender_id: UnitId) -> (f32, f32) {
        let (attacker, defender) = self.get_two_units(attacker_id, defender_id).unwrap();
        let attack_damage = self.get_attack_damage(attacker, defender, attacker.health);
        let new_defender_health = defender.health - attack_damage;

        let counter_attack_damage = if new_defender_health > 0.0 {
            println!(
                "{:?} attacking {:?} ",
                defender.unit_type, attacker.unit_type
            );
            self.get_attack_damage(defender, attacker, new_defender_health)
        } else {
            0.0
        };

        return (counter_attack_damage, attack_damage);
    }

    fn get_attack_damage(&self, attacker: &Unit, defender: &Unit, attacker_health: f32) -> f32 {
        info!(
            "{:?} attacking {:?} ",
            attacker.unit_type, defender.unit_type
        );
        let weapon = self.get_weapon(attacker);
        let full_damage = self.calculate_full_damage(&weapon, &defender.unit_type);
        let attacker_max_health = attacker.unit_type.value().max_health;
        let weakness_scale = attacker_health / attacker_max_health;
        let attack_damage = full_damage * weakness_scale;
        attack_damage
    }

    pub fn get_weapon(&self, unit: &Unit) -> Weapon {
        unit.unit_type
            .value()
            .weapon_one
            .expect("Trying to attack without a weapon")
    }

    pub fn calculate_full_damage(&self, weapon: &Weapon, defender: &UnitType) -> f32 {
        let defender_tags = defender.value().tags;
        let mut bonus_damage = 0.0;
        for bonus in weapon.bonuses.iter() {
            if let Some(bonus) = bonus {
                if defender_tags.iter().any(|maybe_tag| {
                    if let Some(tag) = maybe_tag {
                        *tag == bonus.tag
                    } else {
                        false
                    }
                }) {
                    bonus_damage += bonus.additional_damage
                }
            };
        }
        weapon.base_damage + bonus_damage
    }

    // Will later require knowing which weapon is being used.
    pub fn _get_targets_in_range(&self, attacker_id: UnitId) -> Vec<UnitId> {
        let mut targets_in_range: Vec<UnitId> = vec![];
        for unit in self.units.iter() {
            if self._is_target_in_range(attacker_id, unit.id) {
                targets_in_range.push(unit.id);
            }
        }
        return targets_in_range;
    }

    pub fn _is_target_in_range(&self, _attacker_id: UnitId, _defender_id: UnitId) -> bool {
        // TODO complete this function
        true
    }

    pub fn is_tile_moveable(&self, unit_id: UnitId, x: i32, y: i32) -> bool {
        match self.get_unit(unit_id) {
            Some(unit) => {
                if let Ok((x, y)) = self.is_tile_within_bounds(x, y) {
                    let move_through = self
                        .get_unit_at(x, y)
                        .map_or(true, |unit_2| unit_2.team == unit.team);
                    return move_through;
                };
                return false;
            }
            None => false,
        }
        // Is tile within the map bounds?s
        // Can this unit move over this terrain?
        // Are there any other units already here?
        // Is this blocked by enemy units? (Might require pathfinding?)
    }

    pub fn is_tile_occupied(&self, unit_id: u32, x: u32, y: u32) -> bool {
        if let Some(unit) = self.get_unit_at(x, y) {
            return unit.id != unit_id;
        }
        return false;
    }

    pub fn get_unit_at(&self, x: u32, y: u32) -> Option<&Unit> {
        self.units
            .iter()
            .find(|unit| unit.position.x == x && unit.position.y == y)
    }

    pub fn is_tile_within_bounds(&self, x: i32, y: i32) -> Result<(u32, u32), ()> {
        let x_is_valid = x >= 0 && x < self.map.width() as i32;
        let y_is_valid = y >= 0 && y < self.map.height() as i32;
        if x_is_valid && y_is_valid {
            return Result::Ok((x as u32, y as u32));
        } else {
            return Result::Err(());
        }
    }

    pub fn get_movement_range(&self, _unit_id: &UnitId) -> u32 {
        return 3;
    }

    pub fn get_possible_actions(&self, unit_id: &UnitId) -> Vec<UnitAction> {
        let mut actions: Vec<UnitAction> = vec![];
        let unit = self
            .get_unit(*unit_id)
            .expect(&format!("Could not find unit with id {:?}", unit_id));
        if unit.has_attacked {
            return actions;
        }
        actions.push(UnitAction::Attack);

        if !unit.has_moved {
            actions.push(UnitAction::Move);
        }
        return actions;
    }

    pub fn unit_cannot_act(&self, unit_id: &UnitId) -> bool {
        self.get_possible_actions(unit_id).len() == 0
    }

    pub fn get_units_within_radius(&self, tile: Tile, radius: f32) -> Vec<&Unit> {
        let mut units: Vec<&Unit> = vec![];
        for unit in self.units.iter() {
            if unit.position.distance_to(&tile) <= radius {
                units.push(unit)
            }
        }
        return units;
    }

    pub fn _get_units_within_radius_mut(&mut self, tile: Tile, radius: f32) -> Vec<&mut Unit> {
        let mut units: Vec<&mut Unit> = vec![];
        for unit in self.units.iter_mut() {
            if unit.position.distance_to(&tile) <= radius {
                units.push(unit)
            }
        }
        return units;
    }
}
