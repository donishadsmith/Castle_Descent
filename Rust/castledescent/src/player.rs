use rand::prelude::*;

use std::collections::HashMap;

use crate::castle::Castle;
use crate::merchant::Item;
use crate::movement::Descent;
use crate::utils::filter_possible_coordinates;

pub enum PlayerStatus {
    Active,
    Loss,
    Win,
}

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
    pub status: PlayerStatus,
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
            status: PlayerStatus::Active,
        }
    }

    fn choose_random_coordinate(keys: &mut Vec<(i8, i8, i8)>) -> (i8, i8, i8) {
        let mut rng = rand::rng();

        *keys.choose(&mut rng).unwrap()
    }

    fn select_initial_location(
        castle: &Castle,
        placement: PlayerPlacement,
        current_floor: i8,
    ) -> (i8, i8, i8) {
        let mut keys = match placement {
            PlayerPlacement::Initialize => {
                filter_possible_coordinates(castle, current_floor)
            }
            PlayerPlacement::NextLevel => {
                filter_possible_coordinates(castle, current_floor + 1)
            }
        };

        Self::choose_random_coordinate(&mut keys)
    }
}

impl Descent for Player {
    fn increment_floor(&mut self) -> &mut i8 {
        &mut self.current_position.2
    }
}
