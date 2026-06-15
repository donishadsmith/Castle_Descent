use std::collections::HashMap;

use rand::prelude::*;
use strum::Display;

use crate::{events::prelude::*, merchant::Merchant, utils::prelude::*};

const MIN_FLOORS: i32 = 3;
const MAX_FLOORS: i32 = 5;
const MIN_LENGTH: i32 = 11;
const MAX_LENGTH: i32 = 15;

#[derive(Clone, Copy, Debug, Display, PartialEq)]
pub enum Tile {
    Door(EventID),
    Floor,
    Shop(Merchant),
}

impl Tile {
    pub fn choose_random_event_type() -> i32 {
        let mut rng = rand::rng();
        rng.random_range(1..=10)
    }
}

pub struct Castle {
    pub width: i32,
    pub depth: i32,
    pub floors: i32,
    pub current_floor: i32,
    pub layout: HashMap<Coordinate, Tile>,
}

impl Castle {
    pub fn generate() -> Self {
        let mut rng = rand::rng();

        let valid_values: Vec<i32> = (MIN_LENGTH..=MAX_LENGTH).step_by(2).collect();
        let width = *valid_values.choose(&mut rng).unwrap();
        let depth = *valid_values.choose(&mut rng).unwrap();
        let floors = rng.random_range(MIN_FLOORS..=MAX_FLOORS);

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
        width: i32,
        depth: i32,
        floors: i32,
    ) {
        let base_x_coords: Vec<i32> = (1..width).step_by(2).collect();
        let base_y_coords: Vec<i32> = (1..depth).step_by(2).collect();

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

    fn populate_layout(
        layout: &mut HashMap<Coordinate, Tile>,
        width: i32,
        depth: i32,
        floors: i32,
    ) {
        Self::insert_special_tiles(layout, width, depth, floors);

        let mut monster_hp_range: Vec<i32> = (5..=10).collect();

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

            monster_hp_range = monster_hp_range.iter().map(|x| x + 5).collect::<Vec<i32>>();
        }
    }

    pub fn get_ref_object(&self, coordinate: Coordinate) -> Option<&Tile> {
        self.layout.get(&coordinate)
    }

    pub fn get_mutable_object(&mut self, coordinate: Coordinate) -> Option<&mut Tile> {
        self.layout.get_mut(&coordinate)
    }

    pub fn max_floors(&self) -> i32 {
        self.floors - 1
    }

    pub fn increment_floor(&mut self) {
        self.current_floor += 1
    }
}
