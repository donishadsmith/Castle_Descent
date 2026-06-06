use std::collections::HashMap;

use rand::prelude::*;
use strum::Display;

use crate::movement::Descent;
use crate::utils::prelude::*;

const MIN_FLOORS: i8 = 3;
const MAX_FLOORS: i8 = 6;
const MIN_LENGTH: i8 = 10;
const MAX_LENGTH: i8 = 20;

#[derive(Clone, Copy, Debug, Display)]
pub enum Reveal {
    Monster,
    Fairy,
    Genie,
    Exit,
}

#[derive(Clone, Copy, Debug, Display)]
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
        let width = choose_random_value(MIN_LENGTH..MAX_LENGTH);
        let depth = choose_random_value(MIN_LENGTH..MAX_LENGTH);
        let floors = choose_random_value(MIN_FLOORS..MAX_FLOORS);
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

        // Enums do not naturally have the copy trait and needs to be derived
        // anything that does not implement the copy trait is considered a move
        // which is still a bitwise copy but the compiler considers the new
        // variable the owner of the data. Moves typically for rust types
        // that have a struct on stack that contains a pointer to some data
        // on the heap. Moves are to denote who is responsible for deleting
        // heap data to prevent freeing twice.
        // Copies are bitwise copies for stack only data like primitives, the bytes
        // are independent, for structs that contain pointers, you would end up with
        // two variables that contain the same metadata to the data on the heap which
        // can cause a double free memory issue. Note that not all pointers contain
        // the memory address for data on the heap, it can also contain a memory address
        // to a stack variable that can result in a dangling pointer issue. Essentially,
        // the stack is a last in, first out where stack frames are popped off. If
        // a stack variable points to the address of stack object that it lives longer than,
        // then you risk receiving garbage data. So, you can either have infinite read only copies
        // or one mutable copy to prevent data race conditions. Note that you can bitwise copy data on the
        // heap to the stack.
        // Clones are are considered to be deepcopies and are normally done to copy the
        // heap allocated data and the struct on the stack to create two independent copies
        // that own their own buffers
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
            let x = choose_random_value(0..width);
            let y = choose_random_value(0..depth);

            // Zero reason to explicitly dereference, just doing it to be
            // explicit about the implicit dereferencing of a mutable borrow
            // to modify the object
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
