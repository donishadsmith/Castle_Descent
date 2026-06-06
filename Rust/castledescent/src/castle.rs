use rand::prelude::*;
use std::collections::HashMap;
use strum::Display;

const MIN_FLOORS: i8 = 3;
const MAX_FLOORS: i8 = 6;
const MIN_LENGTH: i8 = 10;
const MAX_LENGTH: i8 = 20;

#[derive(Clone, Debug, Display)]
pub enum Reveal {
    Monster,
    Fairy,
    Genie,
    Exit,
}

#[derive(Clone, Debug, Display)]
pub enum Tile {
    Door(Reveal), //Tile::Door(Reveal::Exit)
    Floor,
}

/* // Can replace the impl trait using #[derive(Clone)]
impl Clone for Reveal {
    fn clone(&self) -> Self {
        match self {
            Reveal::Monster => Reveal::Monster,
            Reveal::Genie => Reveal::Genie,
            Reveal::Fairy => Reveal::Fairy,
            Reveal::Exit => Reveal::Exit,
        }
    }
}

impl Clone for Tile {
    fn clone(&self) -> Self {
        match self {
            Tile::Door(Reveal::Monster) => Tile::Door(Reveal::Monster),
            Tile::Door(Reveal::Genie) => Tile::Door(Reveal::Genie),
            Tile::Door(Reveal::Fairy) => Tile::Door(Reveal::Fairy),
            Tile::Door(Reveal::Exit) => Tile::Door(Reveal::Exit),
            Tile::Floor => Tile::Floor,
        }
    }
}
*/

pub struct Castle {
    pub width: i8,
    pub depth: i8,
    pub floors: i8,
    pub current_floor: i8,
    pub layout: HashMap<(i8, i8, i8), Tile>,
}

impl Castle {
    pub fn generate() -> Self {
        let width = Self::choose_random_value((MIN_LENGTH..MAX_LENGTH).collect());
        let depth = Self::choose_random_value((MIN_LENGTH..MAX_LENGTH).collect());
        let floors = Self::choose_random_value((MIN_FLOORS..MAX_FLOORS).collect());
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

    fn choose_random_value(mut vec: Vec<i8>) -> i8 {
        let mut rng = rand::rng();
        vec.shuffle(&mut rng);

        vec[0]
    }

    fn choose_random_door() -> Tile {
        let mut rng = rand::rng();
        // Assume everything is equally weighted to give greater
        // precedence to Monster (4/6)
        let mut reveals: Vec<Tile> = vec![
            Tile::Door(Reveal::Monster),
            Tile::Door(Reveal::Monster),
            Tile::Door(Reveal::Monster),
            Tile::Door(Reveal::Monster),
            Tile::Door(Reveal::Fairy),
            Tile::Door(Reveal::Genie),
        ];
        reveals.shuffle(&mut rng);

        // This is a move; however, moves require the Copy trait to be implemented
        // which enums do not have unless implemented
        reveals[0].clone()
    }

    fn insert_exits(layout: &mut HashMap<(i8, i8, i8), Tile>, width: i8, depth: i8, floors: i8) {
        for floor in 0..floors {
            let x = Self::choose_random_value((0..width).collect());
            let y = Self::choose_random_value((0..depth).collect());

            layout.insert((x, y, floor), Tile::Door(Reveal::Exit));
        }
    }

    fn populate_layout(
        layout: &mut HashMap<(i8, i8, i8), Tile>,
        width: i8,
        depth: i8,
        floors: i8,
    ) {
        Self::insert_exits(layout, width, depth, floors);

        for i in 0..width {
            for j in 0..depth {
                for k in 0..floors {
                    if layout.contains_key(&(i, j, k)) {
                        continue;
                    }

                    if (i + j) % 2 == 0 {
                        layout.insert((i, j, k), Self::choose_random_door());
                    } else {
                        layout.insert((i, j, k), Tile::Floor);
                    }
                }
            }
        }
    }

    pub fn check_object(&self, x: i8, y: i8, z: i8) -> &str {
        match self.layout.get(&(x, y, z)).unwrap() {
            Tile::Door(Reveal::Monster) => "Monster",
            Tile::Door(Reveal::Genie) => "Genie",
            Tile::Door(Reveal::Fairy) => "Fairy",
            Tile::Door(Reveal::Exit) => "Exit",
            Tile::Floor => "Floor",
        }
    }

    pub fn descend(&mut self) {
        self.current_floor += 1;
    }
}
