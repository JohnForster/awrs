#[derive(Debug)]
pub struct Tile {
    pub x: u32,
    pub y: u32,
}

#[derive(Clone, Copy, Debug)]
pub enum UnitType {
    Infantry,
}

type UnitId = u32;
pub type UnitHp = f32;
type Team = u32;

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

        let mut units_iterator = self.units.iter_mut();
        let attacker = units_iterator
            .find(|u| u.id == attacker_id)
            .expect("Could not find attacker");
        let defender = units_iterator
            .find(|u| u.id == defender_id)
            .expect("Could not find defender");

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
        let mut valid_tiles = vec![];
        let unit = self.get_unit(unit_id).expect("No unit found!");
        let range = 3;

        // Add all tiles within a given movement range
        for dx in (0..=range).rev() {
            for dy in (0..=(range - dx)).rev() {
                valid_tiles.push(Tile {
                    x: unit.position.x + dx,
                    y: unit.position.y + dy,
                })
            }
        }

        valid_tiles = valid_tiles
            .into_iter()
            .filter(|tile| tile.x >= 0 && tile.y >= 0)
            .filter(|tile| tile.x < self.map.width() && tile.y < self.map.height())
            .collect();

        return valid_tiles;
    }
}
