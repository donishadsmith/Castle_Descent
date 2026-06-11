use std::collections::HashMap;

use crate::{
    castle::{Castle, Reveal, Tile},
    merchant::Item,
    utils::{Descent, Status, choose_random_coordinate, filter_possible_coordinates},
};

pub enum PlayerPlacement {
    Initialize,
    NextLevel,
}

pub enum PlayerController {
    Up,
    Down,
    Left,
    Right,
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
                filter_possible_coordinates(&castle.layout, current_floor, Tile::Floor)
            }
            PlayerPlacement::NextLevel => {
                filter_possible_coordinates(&castle.layout, current_floor + 1, Tile::Floor)
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

    // Only increment by grid movements of +- 1 instead of float movement
    pub fn update_position(&mut self, controller: PlayerController, castle: &Castle) {
        let position_tuple = match controller {
            PlayerController::Left => (-1, 0),
            PlayerController::Right => (1, 0),
            PlayerController::Down => (0, -1),
            PlayerController::Up => (0, 1),
        };

        let current_coordinate = (
            self.current_position.0 + position_tuple.0,
            self.current_position.1 + position_tuple.1,
        );
        let object = castle.get_object(
            current_coordinate.0,
            current_coordinate.1,
            self.current_position.2,
        );

        if matches!(object, Some(Tile::Floor)) || matches!(object, Some(Tile::Door(Reveal::Empty)))
        {
            (*self).current_position.0 += position_tuple.0;
            (*self).current_position.1 += position_tuple.1;
        } else if object.is_none() {
            // Out of bounds, perform a wrap. Castle coordinated go from 0 to
            // max - 1, hence modulus should put max to 0 and -1 to max - 1
            // Only player allowed to wrap
            match controller {
                PlayerController::Left | PlayerController::Right => {
                    (*self).current_position.0 = current_coordinate.0.rem_euclid(castle.width);
                }
                PlayerController::Down | PlayerController::Up => {
                    (*self).current_position.1 = current_coordinate.1.rem_euclid(castle.depth);
                }
            }
        }
    }
}

impl Descent for Player {
    fn increment_floor(&mut self) -> &mut i8 {
        &mut self.current_position.2
    }
}
