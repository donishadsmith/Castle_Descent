use std::collections::HashMap;

use rand::prelude::*;
use strum::Display;

use crate::{movement::Descent, utils::prelude::*};

const MIN_FLOORS: i8 = 3;
const MAX_FLOORS: i8 = 6;
const MIN_LENGTH: i8 = 10;
const MAX_LENGTH: i8 = 20;

#[derive(Clone, Copy, Debug, Display, PartialEq)]
pub enum Reveal {
    Monster,
    Fairy,
    Genie,
    Exit,
}

#[derive(Clone, Copy, Debug, Display, PartialEq)]
pub enum Tile {
    Door(Reveal),
    Floor,
}

pub struct Castle {
    pub width: i8,
    pub depth: i8,
    pub floors: i8,
    pub current_floor: i8,
    pub layout: HashMap<(i8, i8, i8), Tile>,
}

impl Castle {
    pub fn generate() -> Self {
        let width = choose_random_value((MIN_LENGTH..MAX_LENGTH).collect());
        let depth = choose_random_value((MIN_LENGTH..MAX_LENGTH).collect());
        let floors = choose_random_value((MIN_FLOORS..MAX_FLOORS).collect());
        let mut layout: HashMap<(i8, i8, i8), Tile> = HashMap::new();
        Self::populate_layout(&mut layout, width, depth, floors);

        Self {
            width,
            depth,
            floors,
            current_floor: 0,
            layout,
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

        // Now that derive is used for the Copy trait, we can now bitwise copy
        // to bitwise copy the first element to another stack memory address
        // if Vec<Tile> = !vec[...] was used, instead it would be a bitwise copy
        // of the data on the heap to a version on the stack
        *reveals.choose(&mut rng).unwrap()
    }

    fn insert_exits(layout: &mut HashMap<(i8, i8, i8), Tile>, width: i8, depth: i8, floors: i8) {
        for floor in 0..floors {
            let x = choose_random_value((0..width).step_by(2).collect());
            let y = choose_random_value((0..depth).step_by(2).collect());

            (*layout).insert((x, y, floor), Tile::Door(Reveal::Exit));
        }
    }

    fn populate_layout(layout: &mut HashMap<(i8, i8, i8), Tile>, width: i8, depth: i8, floors: i8) {
        Self::insert_exits(layout, width, depth, floors);

        for i in 0..width {
            for j in 0..depth {
                for k in 0..floors {
                    if (*layout).contains_key(&(i, j, k)) {
                        continue;
                    }

                    if (i + j) % 2 == 0 {
                        (*layout).insert((i, j, k), Self::choose_random_door());
                    } else {
                        (*layout).insert((i, j, k), Tile::Floor);
                    }
                }
            }
        }
    }

    pub fn check_object(&self, x: i8, y: i8, z: i8) -> &'static str {
        match self.layout.get(&(x, y, z)).unwrap() {
            Tile::Door(Reveal::Monster) => "Monster",
            Tile::Door(Reveal::Genie) => "Genie",
            Tile::Door(Reveal::Fairy) => "Fairy",
            Tile::Door(Reveal::Exit) => "Exit",
            Tile::Floor => "Floor",
        }
    }
}

impl Descent for Castle {
    fn increment_floor(&mut self) -> &mut i8 {
        &mut self.current_floor
    }
}
