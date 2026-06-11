use std::collections::HashMap;

use crate::{
    castle::{Castle, Tile},
    player::{Player, PlayerStatus},
    utils::Descent,
};

enum DistanceMetric {
    Euclidean,
    Chebyshev,
}

pub enum ZombieStatus {
    Roam,
    Frozen,
}

pub struct Zombie {
    pub status: ZombieStatus,
    pub current_position: (i8, i8, i8),
    pub distance_from_player: i8,
}

impl Zombie {
    pub fn spawn(castle: &Castle, player: &Player) -> Self {
        let current_position = Self::select_initial_location(castle, player);
        let distance_from_player = Self::compute_distance(
            &current_position,
            &player.current_position,
            DistanceMetric::Euclidean,
        ) as i8;

        Zombie {
            status: ZombieStatus::Roam,
            current_position,
            distance_from_player,
        }
    }

    fn compute_distance(a: &(i8, i8, i8), b: &(i8, i8, i8), metric: DistanceMetric) -> i16 {
        match metric {
            DistanceMetric::Euclidean => {
                let mut x = (b.0 - a.0) as f32;
                let mut y = (b.1 - a.1) as f32;

                x *= x;
                y *= y;

                (x + y).sqrt().round() as i16
            }
            DistanceMetric::Chebyshev => {
                let x = (b.0 - a.0).abs();
                let y = (b.1 - a.1).abs();

                x.max(y) as i16
            }
        }
    }

    fn select_initial_location(castle: &Castle, player: &Player) -> (i8, i8, i8) {
        let mut distance_hashmap: HashMap<(i8, i8, i8), i16> = HashMap::new();
        for key in castle.layout.keys() {
            if matches!(castle.layout.get(key).unwrap(), Tile::Floor) {
                distance_hashmap.insert(
                    *key,
                    Self::compute_distance(
                        &player.current_position,
                        &key,
                        DistanceMetric::Euclidean,
                    ),
                );
            }
        }

        let max_val = *distance_hashmap.values().max().unwrap();

        distance_hashmap
            .into_iter()
            .find_map(|(key, val)| (val == max_val).then_some(key))
            .unwrap()
    }

    pub fn change_status(&mut self, status: ZombieStatus) {
        self.status = status;
    }

    fn filter_possible_moves(
        possible_moves: Vec<(i8, i8, i8)>,
        castle: &Castle,
    ) -> Vec<(i8, i8, i8)> {
        let mut filtered_moves: Vec<(i8, i8, i8)> = Vec::new();

        for coord in &possible_moves {
            if matches!(castle.layout.get(coord), Some(Tile::Floor)) {
                filtered_moves.push(*coord)
            }
        }

        filtered_moves
    }

    pub fn chase_player(
        &mut self,
        player: &Player,
        predicted_player_position: (i8, i8, i8),
        castle: &Castle,
    ) {
        if matches!(self.status, ZombieStatus::Roam) && matches!(player.status, PlayerStatus::Roam)
        {
            let mut possible_moves: Vec<(i8, i8, i8)> = Vec::new();
            // Dont filter every possible floor, just shift by 1 and determine if floors are valid.
            // Zombie is not allowed to wrap
            for shift in [1, -1] {
                possible_moves.push((
                    self.current_position.0 + shift,
                    self.current_position.1,
                    self.current_position.2,
                ));

                possible_moves.push((
                    self.current_position.0,
                    self.current_position.1 + shift,
                    self.current_position.2,
                ));
            }

            let mut filtered_moves = Self::filter_possible_moves(possible_moves, &castle);
            if filtered_moves.contains(&player.current_position) {
                self.current_position = player.current_position;
            } else {
                let mut distance_hashmap: HashMap<(i8, i8, i8), i16> = HashMap::new();
                for coord in &filtered_moves {
                    distance_hashmap.insert(
                        *coord,
                        Self::compute_distance(
                            &predicted_player_position,
                            &coord,
                            DistanceMetric::Chebyshev,
                        ),
                    );
                }

                let min_val = *distance_hashmap.values().min().unwrap();

                self.current_position = distance_hashmap
                    .into_iter()
                    .find_map(|(key, val)| (val == min_val).then_some(key))
                    .unwrap();
            }
        }
    }
}

impl Descent for Zombie {
    fn increment_floor(&mut self) -> &mut i8 {
        &mut self.current_position.2
    }
}
