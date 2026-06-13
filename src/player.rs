use crate::{
    castle::{Castle, EventID, Tile},
    merchant::Item,
    utils::prelude::*,
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

impl StatusType for PlayerStatus {}

pub struct Player {
    pub hp: i16,
    pub mana: i16,
    pub money: i32,
    pub attack_power: (i16, i16),
    pub current_coordinate: Coordinate,
    pub intended_coordinate: Coordinate, // event is based on the intended coordinate
    pub inventory: HashMap<Item, i16>,
    pub status: PlayerStatus,
}

impl Player {
    pub fn spawn(castle: &Castle) -> Self {
        let current_coordinate =
            Self::select_initial_location(castle, PlayerPlacement::Initialize, 0);
        let intended_coordinate = current_coordinate;

        Self {
            hp: 100,
            mana: 100,
            money: 100,
            attack_power: (1, 5),
            current_coordinate,
            intended_coordinate,
            inventory: HashMap::new(),
            status: PlayerStatus::Roam,
        }
    }

    fn select_initial_location(
        castle: &Castle,
        placement: PlayerPlacement,
        current_floor: i8,
    ) -> Coordinate {
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

    // Only increment by grid movements of +- 1 instead of float movement
    pub fn update_position(&mut self, direction: KeyCode, castle: &Castle) {
        let player_direction = get_direction(direction);

        let new_coordinate = Coordinate::new(
            self.current_coordinate.x + player_direction.x,
            self.current_coordinate.y + player_direction.y,
            self.current_coordinate.z,
        );

        self.intended_coordinate = new_coordinate;
        let object = castle.get_object(new_coordinate);
        if matches!(object, Some(Tile::Floor) | Some(Tile::Door(EventID::Empty))) {
            (*self).current_coordinate.x += player_direction.x;
            (*self).current_coordinate.y += player_direction.y;
        } else if object.is_none() {
            // Out of bounds, perform a wrap. Castle coordinated go from 0 to
            // max - 1, hence modulus should put max to 0 and -1 to max - 1
            // Only player allowed to wrap
            match direction {
                KeyCode::Left | KeyCode::Right => {
                    (*self).current_coordinate.x = new_coordinate.x.rem_euclid(castle.width);
                }
                KeyCode::Down | KeyCode::Up => {
                    (*self).current_coordinate.y = new_coordinate.y.rem_euclid(castle.depth);
                }
                _ => (),
            }
        }
    }
}

impl Entity for Player {}

impl EntityStatus for Player {
    type Status = PlayerStatus;
    fn current_status(&mut self) -> &mut PlayerStatus {
        &mut self.status
    }
}

impl Descent for Player {
    fn increment_floor(&mut self) -> &mut i8 {
        &mut self.current_coordinate.z
    }
}
