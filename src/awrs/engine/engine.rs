use bevy::prelude::{debug, info, EventWriter};

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

#[derive(Clone, Copy, Debug)]
pub enum UnitType {
    Infantry,
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

pub enum UnitAction {
    Move,
    Attack,
    _Capture,
    _Join,
    _SelfDestruct,
    _Resupply,
}

pub enum Command {
    Move {
        unit_id: UnitId,
        tiles: Vec<Tile>,
    },
    Attack {
        attacker_id: UnitId,
        defender_id: UnitId,
    },
    EndTurn,
}

#[derive(Debug)]
pub struct ScenarioState {
    pub map: ScenarioMap,
    pub units: Vec<Unit>,
    pub active_team: u32,
    pub teams: Vec<u32>,
}

pub enum CommandStatus {
    Ok,
    Partial,
    Err,
}

pub enum CommandResult {
    Move {
        status: CommandStatus,
        tiles: Vec<Tile>,
    },
    Attack {
        status: CommandStatus,
        unit_hp: Vec<(UnitId, UnitHp)>,
    },
    EndTurn {
        status: CommandStatus,
        new_active_team: Team,
    },
}

impl ScenarioState {
    pub fn execute(&mut self, command: Command) -> CommandResult {
        match command {
            Command::Move { unit_id, tiles } => self.unit_move(unit_id, tiles),
            Command::Attack {
                attacker_id,
                defender_id,
            } => self.attack(attacker_id, defender_id),
            Command::EndTurn => self.end_turn(),
        }
    }

    fn unit_move(&mut self, id: UnitId, tiles: Vec<Tile>) -> CommandResult {
        let mut units_iterator = self.units.iter_mut();

        let mut unit = units_iterator
            .find(|u| u.id == id)
            .expect(format!("No unit found with id {}", id).as_str());

        if unit.has_moved {
            return CommandResult::Move {
                status: CommandStatus::Err,
                tiles: vec![unit.position],
            };
        }

        let mut successful_moves: Vec<Tile> = vec![];
        let mut status = CommandStatus::Err;

        for Tile { x, y } in tiles {
            // TODO - Check valid
            //   Adjacent
            //   Terrain/Fuel
            //   Hasn't already moved

            // Check that each tile is free
            let maybe_blocking_unit =
                units_iterator.find(|u| u.position.x == x && u.position.y == y);

            match maybe_blocking_unit {
                None => {}
                Some(_unit) => {
                    status = CommandStatus::Partial;
                    break;
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

        if attacker.has_attacked {
            return CommandResult::Attack {
                status: CommandStatus::Err,
                unit_hp: vec![
                    (attacker.id, attacker.health),
                    (defender.id, defender.health),
                ],
            };
        }

        // Calculate damage
        let (attacker_damage, defender_damage) = (2.0, 4.0);
        attacker.health -= attacker_damage;
        defender.health -= defender_damage;

        attacker.has_attacked = true;

        return CommandResult::Attack {
            status: CommandStatus::Ok,
            unit_hp: vec![
                (attacker.id, attacker.health),
                (defender.id, defender.health),
            ],
        };
    }

    fn end_turn(&mut self) -> CommandResult {
        info!("Ending turn");
        let new_active_team = (self.active_team + 1) % (self.teams.len() as u32);
        self.active_team = new_active_team;
        for mut unit in self.units.iter_mut() {
            unit.has_attacked = false;
            unit.has_moved = false;
        }
        return CommandResult::EndTurn {
            status: CommandStatus::Ok,
            new_active_team,
        };
    }

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

    // Will later require knowing which weapon is being used.
    pub fn get_targets_in_range(&self, attacker_id: UnitId) -> Vec<UnitId> {
        let mut targets_in_range: Vec<UnitId> = vec![];
        for unit in self.units.iter() {
            if self.is_target_in_range(attacker_id, unit.id) {
                targets_in_range.push(unit.id);
            }
        }
        return targets_in_range;
    }

    pub fn is_target_in_range(&self, _attacker_id: UnitId, _defender_id: UnitId) -> bool {
        // TODO complete this function
        true
    }

    pub fn is_tile_moveable(&self, unit_id: UnitId, x: i32, y: i32) -> bool {
        // Is tile within the map bounds?
        self.is_tile_within_bounds(x, y) &&
        // Can this unit move over this terrain?
        // Are there any other units already here?
        !self.is_tile_occupied(unit_id, x as u32, y as u32)
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

    pub fn is_tile_within_bounds(&self, x: i32, y: i32) -> bool {
        let x_is_valid = x >= 0 && x < self.map.width() as i32;
        let y_is_valid = y >= 0 && y < self.map.height() as i32;

        return x_is_valid && y_is_valid;
    }

    pub fn get_movement_range(&self, _unit_id: &UnitId) -> u32 {
        return 3;
    }

    // pub fn get_possible_actions(&self, unit_id: &UnitId) -> Vec<UnitAction> {
    //     let actions: Vec<UnitAction> = vec![];
    //     if
    // }
}
