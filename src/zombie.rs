use std::collections::HashMap;

use crate::{
    castle::{Castle, Tile},
    player::{Player, PlayerStatus},
    utils::prelude::*,
};

enum DistanceMetric {
    Euclidean,
    Chebyshev,
}

pub enum ZombieStatus {
    Roam,
    Frozen,
}

impl StatusType for ZombieStatus {}

pub struct Zombie {
    pub status: ZombieStatus,
    pub current_coordinate: Coordinate,
    pub distance_from_player: i32,
    pub accumulator: f32,
}

impl Zombie {
    pub fn spawn(castle: &Castle, player: &Player) -> Self {
        let current_coordinate = Self::select_initial_location(castle, player, 0);
        let distance_from_player = Self::compute_distance(
            &current_coordinate,
            &player.current_coordinate,
            DistanceMetric::Euclidean,
        );

        Zombie {
            status: ZombieStatus::Roam,
            current_coordinate,
            distance_from_player,
            accumulator: 0.0,
        }
    }

    fn compute_distance(a: &Coordinate, b: &Coordinate, metric: DistanceMetric) -> i32 {
        match metric {
            DistanceMetric::Euclidean => {
                let dx = b.x - a.x;
                let dy = b.y - a.y;

                dx * dx + dy * dy
            }
            DistanceMetric::Chebyshev => {
                let x = (b.x - a.x).abs();
                let y = (b.y - a.y).abs();

                x.max(y)
            }
        }
    }

    pub fn select_initial_location(castle: &Castle, player: &Player, floor: i32) -> Coordinate {
        let mut distance_hashmap: HashMap<Coordinate, i32> = HashMap::new();
        for key in castle.layout.keys() {
            if key.z == floor && matches!(castle.layout.get(key).unwrap(), Tile::Floor) {
                distance_hashmap.insert(
                    *key,
                    Self::compute_distance(
                        &player.current_coordinate,
                        key,
                        DistanceMetric::Euclidean,
                    ),
                );
            }
        }

        distance_hashmap
            .into_iter()
            .max_by_key(|(c, v)| (*v, c.x, c.y))
            .unwrap()
            .0
    }

    fn filter_possible_moves(&self, castle: &Castle) -> Vec<Coordinate> {
        let mut possible_moves: Vec<Coordinate> = Vec::new();

        for shift in [1, -1] {
            possible_moves.push(Coordinate::new(
                self.current_coordinate.x + shift,
                self.current_coordinate.y,
                self.current_coordinate.z,
            ));

            possible_moves.push(Coordinate::new(
                self.current_coordinate.x,
                self.current_coordinate.y + shift,
                self.current_coordinate.z,
            ));
        }

        let mut filtered_moves: Vec<Coordinate> = Vec::new();

        for coord in &possible_moves {
            if matches!(castle.layout.get(coord), Some(Tile::Floor)) {
                filtered_moves.push(*coord)
            }
        }

        filtered_moves
    }

    pub fn chase_player(&mut self, player: &Player, castle: &Castle) {
        if self.current_coordinate == player.current_coordinate {
            return;
        }

        if matches!(self.status, ZombieStatus::Roam) && matches!(player.status, PlayerStatus::Roam)
        {
            let filtered_moves = self.filter_possible_moves(castle);
            if filtered_moves.contains(&player.current_coordinate) {
                self.current_coordinate = player.current_coordinate;
            } else {
                let mut distance_hashmap: HashMap<Coordinate, i32> = HashMap::new();
                for coord in &filtered_moves {
                    distance_hashmap.insert(
                        *coord,
                        Self::compute_distance(
                            &player.current_coordinate,
                            coord,
                            DistanceMetric::Chebyshev,
                        ),
                    );
                }

                self.current_coordinate = filtered_moves
                    .into_iter()
                    .min_by_key(|c| {
                        (
                            Self::compute_distance(
                                &player.current_coordinate,
                                c,
                                DistanceMetric::Euclidean,
                            ),
                            c.x,
                            c.y,
                        )
                    })
                    .unwrap();
            }
        }
    }

    pub fn random_move(&mut self, castle: &Castle) {
        let mut possible_moves = self.filter_possible_moves(castle);
        possible_moves.push(self.current_coordinate);

        self.current_coordinate = choose_random_coordinate(&mut possible_moves);
    }
}

impl Entity for Zombie {}

impl Descent for Zombie {
    fn increment_floor(&mut self) -> &mut i32 {
        &mut self.current_coordinate.z
    }
}

impl EntityStatus for Zombie {
    type Status = ZombieStatus;

    fn current_status(&mut self) -> &mut ZombieStatus {
        &mut self.status
    }
}
