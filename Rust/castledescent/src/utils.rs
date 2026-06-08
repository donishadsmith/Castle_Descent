use crate::castle::Castle;
use crate::castle::Tile;

pub mod prelude {
    use rand::prelude::*;
    use std::ops::Range;

    pub fn choose_random_value(range: Range<i8>) -> i8 {
        let mut rng = rand::rng();

        rng.random_range(range)
    }
}

pub fn filter_possible_coordinates(castle: &Castle, current_floor: i8) -> Vec<(i8, i8, i8)> {
    let mut keys: Vec<(i8, i8, i8)> = Vec::new();
    for key in castle.layout.keys() {
        if key.2 == current_floor && matches!(castle.layout.get(key).unwrap(), Tile::Floor) {
            keys.push(key.clone())
        } else {
            continue;
        }
    }

    keys
}
