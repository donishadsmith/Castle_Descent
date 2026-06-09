use rand::prelude::*;

use crate::castle::{Castle, Tile};

pub enum Status {
    Active,
    Lose,
    Win,
}

pub mod prelude {
    use rand::prelude::*;

    pub fn choose_random_value(mut vec: Vec<i8>) -> i8 {
        let mut rng = rand::rng();
        vec.shuffle(&mut rng);

        vec[0]
    }
}

pub fn choose_random_coordinate(keys: &mut Vec<(i8, i8, i8)>) -> (i8, i8, i8) {
    let mut rng = rand::rng();

    *keys.choose(&mut rng).unwrap()
}

pub fn filter_possible_coordinates(castle: &Castle, current_floor: i8) -> Vec<(i8, i8, i8)> {
    let filtered_keys = castle
        .layout
        .keys()
        .into_iter()
        .filter_map(|key| {
            (key.2 == current_floor && matches!(castle.layout.get(&key).unwrap(), Tile::Floor))
                .then_some(key.clone())
        })
        .collect();

    filtered_keys
}
