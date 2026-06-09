use std::collections::HashMap;

use crate::{
    castle::{Castle, Tile},
    merchant::Item,
    movement::Descent,
    utils::{Status, choose_random_coordinate, filter_possible_coordinates},
};

pub enum PlayerPlacement {
    Initialize,
    NextLevel,
}

pub struct Player {
    pub hp: i16,
    pub mana: i16,
    pub money: i32,
    pub attack_power: (i16, i16),
    pub current_position: (i8, i8, i8),
    pub inventory: HashMap<Item, i16>,
    pub status: Status,
    pub unicode: &'static str,
}

impl Player {
    pub fn spawn(castle: &Castle) -> Self {
        let current_position =
            Self::select_initial_location(castle, PlayerPlacement::Initialize, 0);

        Self {
            hp: 100,
            mana: 100,
            money: 100,
            attack_power: (1, 5),
            current_position,
            inventory: HashMap::new(),
            status: Status::Active,
            unicode: &"\u{1F93A}",
        }
    }

    fn select_initial_location(
        castle: &Castle,
        placement: PlayerPlacement,
        current_floor: i8,
    ) -> (i8, i8, i8) {
        let mut keys = match placement {
            PlayerPlacement::Initialize => {
                filter_possible_coordinates(castle, current_floor, Tile::Floor)
            }
            PlayerPlacement::NextLevel => {
                filter_possible_coordinates(castle, current_floor + 1, Tile::Floor)
            }
        };

        choose_random_coordinate(&mut keys)
    }

    pub fn change_status(&mut self, status: Status) {
        self.status = match status {
            Status::Active => Status::Active,
            Status::Win => Status::Win,
            Status::Lose => Status::Lose,
        }
    }

    pub fn caught(&mut self) {
        self.status = Status::Lose
    }
}

impl Descent for Player {
    fn increment_floor(&mut self) -> &mut i8 {
        &mut self.current_position.2
    }
}
