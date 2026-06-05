use rand::prelude::*;
use std::{collections::HashMap, ops::Range};

enum Item {
    CrystalBall,
    Potion,
    MagnifyingGlass,
}

struct Player {
    hp: i8,
    mana: i8,
    money: i32,
    attack_power: Range<i8>,
    current_position: (i8, i8, i8),
    inventory: HashMap<i8, Item>,
}

struct Zombie {
    halt: bool,
    current_position: (i8, i8, i8),
    distance_from_player: u32,
}

struct Castle {}
// -> Option<i8> to return nums.copied()
/// Gets a random castle dimension between 10 and 20.
///
/// # Examples
/// ```
/// let (w, h) = get_castle_dims();
/// ```
fn get_castle_dims() -> (i8, i8) {
    let mut rng = rand::rng();
    let mut dims: Vec<i8> = (10..=20).collect();
    dims.shuffle(&mut rng);

    (dims[0], dims[1])
}

/*impl Castle {
    // Called as Castle::generate(), like Python's classmethod / __init__.
    fn generate() -> Castle {
        // ... do all the random spawning, build the tiles Vec ...
        Castle { tiles, width, height, floors, current_floor: 0 }
    }

    // Methods (note: `&self` / `&mut self`) — operate on an existing castle.
    //fn get(&self, x: usize, y: usize, z: usize) -> &Tile { /* ... */ }
    //fn descend(&mut self) { self.current_floor += 1; }

    }
*/

fn main() {
    let (width, height) = get_castle_dims();
    println!("{:?}", (width, height))
}

