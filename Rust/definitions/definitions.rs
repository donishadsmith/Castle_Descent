use std::{collections::HashMap, ops::Range};

enum Item {
    CrystalBall,
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

fn main() {}
