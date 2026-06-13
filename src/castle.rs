use std::collections::HashMap;

use rand::prelude::*;
use strum::Display;

use crate::{events::prelude::*, merchant::Merchant, utils::prelude::*};

const MIN_FLOORS: i8 = 3;
const MAX_FLOORS: i8 = 6;
const MIN_LENGTH: i8 = 10;
const MAX_LENGTH: i8 = 20;

#[derive(Clone, Copy, Debug, Display, PartialEq)]
pub enum Tile {
    Door(EventID),
    Floor,
    Shop(Merchant),
}

impl Tile {
    pub fn choose_random_event_type() -> i8 {
        let mut rng = rand::rng();
        rng.random_range(1..=10)
    }
}

pub struct Castle {
    pub width: i8,
    pub depth: i8,
    pub floors: i8,
    pub current_floor: i8,
    pub layout: HashMap<Coordinate, Tile>,
}

impl Castle {
    pub fn generate() -> Self {
        let mut rng = rand::rng();

        let width = rng.random_range(MIN_LENGTH..MAX_LENGTH);
        let depth = rng.random_range(MIN_LENGTH..MAX_LENGTH);
        let floors = rng.random_range(MIN_FLOORS..MAX_FLOORS);

        let mut layout: HashMap<Coordinate, Tile> = HashMap::new();
        Self::populate_layout(&mut layout, width, depth, floors);

        Self {
            width,
            depth,
            floors,
            current_floor: 0,
            layout,
        }
    }

    fn insert_special_tiles(
        layout: &mut HashMap<Coordinate, Tile>,
        width: i8,
        depth: i8,
        floors: i8,
    ) {
        let base_x_coords: Vec<i8> = (1..width).step_by(2).collect();
        let base_y_coords: Vec<i8> = (1..depth).step_by(2).collect();

        for floor in 0..floors {
            let exit_x = choose_random_value(&base_x_coords);
            let exit_y = choose_random_value(&base_y_coords);

            layout.insert(
                Coordinate::new(exit_x, exit_y, floor),
                Tile::Door(EventID::Exit),
            );

            let mut merch_x_coords = base_x_coords.clone();
            let mut merch_y_coords = base_y_coords.clone();

            merch_x_coords.retain(|&x| x != exit_x);
            merch_y_coords.retain(|&y| y != exit_y);

            let merch_x = choose_random_value(&merch_x_coords);
            let merch_y = choose_random_value(&merch_y_coords);

            layout.insert(
                Coordinate::new(merch_x, merch_y, floor),
                Tile::Shop(Merchant {}),
            );
        }
    }

    fn populate_layout(layout: &mut HashMap<Coordinate, Tile>, width: i8, depth: i8, floors: i8) {
        Self::insert_special_tiles(layout, width, depth, floors);

        let mut monster_hp_range: Vec<i8> = (5..=10).collect();

        for z in 0..floors {
            for x in 0..width {
                for y in 0..depth {
                    if (*layout).contains_key(&Coordinate::new(x, y, z)) {
                        continue;
                    }

                    if x % 2 != 0 && y % 2 != 0 {
                        let event_data = match Tile::choose_random_event_type() {
                            1..=8 => {
                                let current_monster_hp = choose_random_value(&monster_hp_range);
                                Tile::Door(EventID::MonsterEvent(Monster::spawn(
                                    current_monster_hp,
                                )))
                            }
                            9 => Tile::Door(EventID::FairyEvent(Fairy::spawn())),
                            _ => Tile::Door(EventID::GenieEvent(Genie::spawn())),
                        };
                        (*layout).insert(Coordinate::new(x, y, z), event_data);
                    } else {
                        (*layout).insert(Coordinate::new(x, y, z), Tile::Floor);
                    }
                }
            }

            // Increment by 5, i8 range allows -127 to 128
            // Max number of floors is 6 so max hp will 60 for monsters, within range
            monster_hp_range = monster_hp_range.iter().map(|x| x + 5).collect::<Vec<i8>>();
        }
    }

    pub fn get_object(&self, coordinate: Coordinate) -> Option<&Tile> {
        self.layout.get(&coordinate)
    }

    pub fn get_mutable_object(&mut self, coordinate: Coordinate) -> Option<&mut Tile> {
        self.layout.get_mut(&coordinate)
    }

    pub fn max_floors(&self) -> i8 {
        self.floors - 1
    }
}

impl Descent for Castle {
    fn increment_floor(&mut self) -> &mut i8 {
        &mut self.current_floor
    }
}
