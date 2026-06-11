use std::collections::HashMap;

use rand::prelude::*;
use strum::Display;

use crate::utils::{Descent, filter_possible_coordinates, prelude::*};

const MIN_FLOORS: i8 = 3;
const MAX_FLOORS: i8 = 6;
const MIN_LENGTH: i8 = 10;
const MAX_LENGTH: i8 = 20;

#[derive(Clone, Copy, Debug, Display, PartialEq)]
pub enum Reveal {
    Monster,
    Fairy,
    Genie,
    Empty,
    Exit,
}

#[derive(Clone, Copy, Debug, Display, PartialEq)]
pub enum Tile {
    Door(Reveal),
    Floor,
    Merchant,
}

pub struct Castle {
    pub width: i8,
    pub depth: i8,
    pub floors: i8,
    pub current_floor: i8,
    pub layout: HashMap<(i8, i8, i8), Tile>,
    pub monster_data: HashMap<(i8, i8, i8), i8>,
}

impl Castle {
    pub fn generate() -> Self {
        let width = choose_random_value((MIN_LENGTH..MAX_LENGTH).collect());
        let depth = choose_random_value((MIN_LENGTH..MAX_LENGTH).collect());
        let floors = choose_random_value((MIN_FLOORS..MAX_FLOORS).collect());
        let mut layout: HashMap<(i8, i8, i8), Tile> = HashMap::new();
        Self::populate_layout(&mut layout, width, depth, floors);
        let monster_data = Self::generate_monster_data(&layout, floors);

        Self {
            width,
            depth,
            floors,
            current_floor: 0,
            layout,
            monster_data,
        }
    }

    fn choose_random_door() -> Tile {
        let mut rng = rand::rng();
        // Assume everything is equally weighted to give greater
        // precedence to Monster (4/6)
        let reveals: [Tile; 6] = [
            Tile::Door(Reveal::Monster),
            Tile::Door(Reveal::Monster),
            Tile::Door(Reveal::Monster),
            Tile::Door(Reveal::Monster),
            Tile::Door(Reveal::Fairy),
            Tile::Door(Reveal::Genie),
        ];

        *reveals.choose(&mut rng).unwrap()
    }

    fn insert_exits(layout: &mut HashMap<(i8, i8, i8), Tile>, width: i8, depth: i8, floors: i8) {
        for floor in 0..floors {
            let x = choose_random_value((1..width).step_by(2).collect());
            let y = choose_random_value((1..depth).step_by(2).collect());

            (*layout).insert((x, y, floor), Tile::Door(Reveal::Exit));
        }
    }

    fn insert_merchants(
        layout: &mut HashMap<(i8, i8, i8), Tile>,
        width: i8,
        depth: i8,
        floors: i8,
    ) {
        for floor in 0..floors {
            let mut possible_x_coordinates: Vec<i8> = (1..width).step_by(2).collect();
            let mut possible_y_coordinates: Vec<i8> = (1..depth).step_by(2).collect();

            let exit_coordinate =
                filter_possible_coordinates(&(*layout), floor, Tile::Door(Reveal::Exit))[0];
            possible_x_coordinates.retain(|&x| x != exit_coordinate.0);
            possible_y_coordinates.retain(|&y| y != exit_coordinate.1);

            let x = choose_random_value(possible_x_coordinates);
            let y = choose_random_value(possible_y_coordinates);

            (*layout).insert((x, y, floor), Tile::Merchant);
        }
    }

    fn populate_layout(layout: &mut HashMap<(i8, i8, i8), Tile>, width: i8, depth: i8, floors: i8) {
        Self::insert_exits(layout, width, depth, floors);
        Self::insert_merchants(layout, width, depth, floors);

        for i in 0..width {
            for j in 0..depth {
                for k in 0..floors {
                    if (*layout).contains_key(&(i, j, k)) {
                        continue;
                    }

                    if i % 2 != 0 && j % 2 != 0 {
                        (*layout).insert((i, j, k), Self::choose_random_door());
                    } else {
                        (*layout).insert((i, j, k), Tile::Floor);
                    }
                }
            }
        }
    }

    fn generate_monster_data(
        layout: &HashMap<(i8, i8, i8), Tile>,
        floors: i8,
    ) -> HashMap<(i8, i8, i8), i8> {
        let mut monster_data = HashMap::<(i8, i8, i8), i8>::new();
        let mut monster_hp: Vec<i8> = (1..=5).collect();

        for floor in 0..floors {
            let coordinates =
                filter_possible_coordinates(&(*layout), floor, Tile::Door(Reveal::Monster));
            for coordinate in coordinates {
                monster_data.insert(coordinate, choose_random_value(monster_hp.clone()));
            }

            // Increment by 5, i8 range allows -127 to 128
            // Mac number of floors is 6 so max hp will 35 for monsters, within range
            monster_hp = monster_hp.iter().map(|x| x + 5).collect::<Vec<i8>>();
        }

        monster_data
    }

    pub fn get_object(&self, x: i8, y: i8, z: i8) -> Option<Tile> {
        self.layout.get(&(x, y, z)).copied()
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
