use crate::{
    castle::{Castle, Reveal, Tile},
    merchant::Item,
    utils::{Descent, choose_random_coordinate, filter_possible_coordinates, get_direction},
};
use macroquad::input::KeyCode;
use std::collections::HashMap;

pub enum PlayerPlacement {
    Initialize,
    NextLevel,
}

pub enum PlayerStatus {
    Roam,
    Win,
    Lose,
    Event,
    Inventory,
    Hide,
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
            status: PlayerStatus::Roam,
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

    pub fn change_status(&mut self, status: PlayerStatus) {
        self.status = match status {
            PlayerStatus::Roam => PlayerStatus::Roam,
            PlayerStatus::Win => PlayerStatus::Win,
            PlayerStatus::Lose => PlayerStatus::Lose,
            PlayerStatus::Inventory => PlayerStatus::Inventory,
            PlayerStatus::Event => PlayerStatus::Event,
            PlayerStatus::Hide => PlayerStatus::Hide,
        }
    }

    // Only increment by grid movements of +- 1 instead of float movement
    pub fn update_position(&mut self, direction: KeyCode, castle: &Castle) {
        let player_direction = get_direction(direction);

        let current_coordinate = (
            self.current_position.0 + player_direction.0,
            self.current_position.1 + player_direction.1,
        );
        let object = castle.get_object(
            current_coordinate.0,
            current_coordinate.1,
            self.current_position.2,
        );

        if matches!(object, Some(Tile::Floor) | Some(Tile::Door(Reveal::Empty))) {
            (*self).current_position.0 += player_direction.0;
            (*self).current_position.1 += player_direction.1;
        } else if object.is_none() {
            // Out of bounds, perform a wrap. Castle coordinated go from 0 to
            // max - 1, hence modulus should put max to 0 and -1 to max - 1
            // Only player allowed to wrap
            match direction {
                KeyCode::Left | KeyCode::Right => {
                    (*self).current_position.0 = current_coordinate.0.rem_euclid(castle.width);
                }
                KeyCode::Down | KeyCode::Up => {
                    (*self).current_position.1 = current_coordinate.1.rem_euclid(castle.depth);
                }
                _ => (),
            }
        }
    }
}

impl Descent for Player {
    fn increment_floor(&mut self) -> &mut i8 {
        &mut self.current_position.2
    }
}
