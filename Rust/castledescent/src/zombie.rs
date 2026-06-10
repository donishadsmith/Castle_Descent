use std::collections::HashMap;

use crate::{
    castle::{Castle, Tile},
    movement::Descent,
    player::Player,
    utils::Status,
};

pub enum ZombieStatus {
    NotFrozen(Status),
    Frozen,
}

pub struct Zombie {
    pub status: ZombieStatus,
    pub current_position: (i8, i8, i8),
    pub distance_from_player: i8,
    pub unicode: &'static str,
}

impl Zombie {
    pub fn spawn(castle: &Castle, player: &Player) -> Self {
        let current_position = Self::select_initial_location(castle, player);
        let distance_from_player =
            Self::approximate_euclidean_distance(&current_position, &player.current_position) as i8;

        Zombie {
            status: ZombieStatus::NotFrozen(Status::Active),
            current_position,
            distance_from_player,
            unicode: &"\u{1F9DF}",
        }
    }

    pub fn approximate_euclidean_distance(a: &(i8, i8, i8), b: &(i8, i8, i8)) -> i16 {
        let mut x = (b.0 - a.0) as f32;
        let mut y = (b.1 - a.1) as f32;

        x *= x;
        y *= y;

        (x + y).sqrt().round() as i16
    }

    fn select_initial_location(castle: &Castle, player: &Player) -> (i8, i8, i8) {
        let mut distance_hashmap: HashMap<(i8, i8, i8), i16> = HashMap::new();
        for key in castle.layout.keys() {
            if matches!(castle.layout.get(key).unwrap(), Tile::Floor) {
                distance_hashmap.insert(
                    key.clone(),
                    Self::approximate_euclidean_distance(&player.current_position, &key),
                );
            }
        }

        let max_val = distance_hashmap.values().max().unwrap().clone();

        // Will never be None
        distance_hashmap
            .into_iter()
            .find_map(|(key, val)| (val == max_val).then_some(key))
            .unwrap()
    }

    pub fn change_status(&mut self, status: ZombieStatus) {
        self.status = match status {
            ZombieStatus::NotFrozen(Status::Active) => ZombieStatus::NotFrozen(Status::Active),
            ZombieStatus::NotFrozen(Status::Win) => ZombieStatus::NotFrozen(Status::Win),
            ZombieStatus::NotFrozen(Status::Lose) => ZombieStatus::NotFrozen(Status::Lose),
            ZombieStatus::Frozen => ZombieStatus::Frozen,
        }
    }
}

impl Descent for Zombie {
    fn increment_floor(&mut self) -> &mut i8 {
        &mut self.current_position.2
    }
}
