use crate::{
    castle::{Castle, Reveal, Tile},
    player::Player,
    zombie::Zombie,
};
use macroquad::input::KeyCode;
use rand::prelude::*;
use std::collections::HashMap;

pub trait Descent {
    fn increment_floor(&mut self) -> &mut i8;

    fn descend(&mut self) {
        *self.increment_floor() += 1
    }
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

pub fn filter_possible_coordinates(
    layout: &HashMap<(i8, i8, i8), Tile>,
    current_floor: i8,
    filter_type: Tile,
) -> Vec<(i8, i8, i8)> {
    let filtered_keys: Vec<(i8, i8, i8)> = layout
        .iter()
        .filter_map(|(key, tile)| (key.2 == current_floor && tile == &filter_type).then_some(*key))
        .collect();

    filtered_keys
}

pub fn get_direction(direction: KeyCode) -> (i8, i8, i8) {
    match direction {
        KeyCode::Left => (-1, 0, 0),
        KeyCode::Right => (1, 0, 0),
        KeyCode::Down => (0, 1, 0),
        KeyCode::Up => (0, -1, 0),
        _ => (0, 0, 0),
    }
}
