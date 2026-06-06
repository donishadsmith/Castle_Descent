use crate::merchant::Item;
use std::{collections::HashMap, ops::Range};

enum PlayerStatus {
    Active,
    Loss,
    Win,
}

pub struct Player {
    hp: i8,
    mana: i8,
    money: i32,
    attack_power: Range<i8>,
    current_position: (i8, i8, i8),
    inventory: HashMap<i8, Item>,
    status: PlayerStatus,
}
