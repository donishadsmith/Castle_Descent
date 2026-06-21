use std::collections::HashMap;

use macroquad::prelude::*;
use strum::Display;

use crate::{
    castle::{Castle, Tile},
    item::Item,
    player::{Player, PlayerStatus},
    utils::prelude::*,
};

pub enum DistanceMetric {
    Euclidean,
    Chebyshev,
}

#[derive(PartialEq, Eq, Debug, Display)]
pub enum ZombieStatus {
    Roam,
    Frozen,
}

impl StatusType for ZombieStatus {}

pub struct Zombie {
    pub status: ZombieStatus,
    pub current_coordinate: Coordinate,
    pub freeze_timer: f32,
    pub accumulator: f32,
}

impl Zombie {
    pub fn spawn(castle: &Castle, player: &Player) -> Self {
        let current_coordinate = Self::select_initial_location(castle, player, 0);

        Self {
            status: ZombieStatus::Roam,
            current_coordinate,
            freeze_timer: 0.0,
            accumulator: 0.0,
        }
    }

    /// Assuming z is the same
    pub fn compute_distance(a: &Coordinate, b: &Coordinate, metric: DistanceMetric) -> i32 {
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
        if self.current_coordinate == player.current_coordinate
            || !(self.status == ZombieStatus::Roam && player.status == PlayerStatus::Roam)
        {
            return;
        }

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

    pub fn wander(&mut self, castle: &Castle) {
        if self.status == ZombieStatus::Frozen {
            return;
        }

        let mut possible_moves = self.filter_possible_moves(castle);
        possible_moves.push(self.current_coordinate);

        self.current_coordinate = choose_random_coordinate(&mut possible_moves);
    }

    pub fn freeze(&mut self, player: &mut Player, game_state: &GameState, dt: f32) {
        let freeze_active = !player.in_inventory()
            && player.effects.freeze_zombie()
            && *game_state == GameState::Active
            && !player.in_event();

        if !freeze_active {
            return;
        }

        if self.status != ZombieStatus::Frozen {
            self.update_status(ZombieStatus::Frozen);

            for _ in 0..player.effects.count(Item::Hourglass) {
                self.freeze_timer += player.effects.freeze_time();
            }
        }

        self.decrement_timer(dt);

        if self.status == ZombieStatus::Roam {
            player.effects.inactivate(Item::Hourglass);
        }
    }

    fn decrement_timer(&mut self, dt: f32) {
        self.freeze_timer -= dt;
        if self.freeze_timer <= 0.0 {
            self.freeze_timer = 0.0;
            self.update_status(ZombieStatus::Roam);
        }

        draw_text(
            format!(
                "Zombie Freeze Timer: {} seconds",
                (self.freeze_timer as i32)
            ),
            screen_width() / 2.0 * 0.90,
            screen_height() * 0.95,
            20.0,
            WHITE,
        );
    }
}

impl Entity for Zombie {
    type Status = ZombieStatus;

    fn current_status(&mut self) -> &mut ZombieStatus {
        &mut self.status
    }
}


// Eventually add more tests plus tests for castle and player
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compute_distance() {

        let a = Coordinate::new(1, 2, 0);
        let b = Coordinate::new(4, 6, 0);

        let mut distance = Zombie::compute_distance(&a, &b, DistanceMetric::Euclidean);
        assert_eq!(distance, 25);

        distance = Zombie::compute_distance(&a, &b, DistanceMetric::Chebyshev);
        assert_eq!(distance, 4);
    }
}