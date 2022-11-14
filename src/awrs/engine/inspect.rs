pub impl ScenarioState {
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
