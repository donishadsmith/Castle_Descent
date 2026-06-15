use crate::{
    castle::{Castle, Tile},
    controller::Controller,
    events::prelude::EventID,
    merchant::Item,
    utils::prelude::*,
};
use macroquad::input::KeyCode;
use std::collections::HashMap;

#[derive(PartialEq, Debug)]
pub enum PlayerStatus {
    Roam,
    Win,
    Lose,
    Event,
    Inventory,
    Hide,
    Shop,
}

impl StatusType for PlayerStatus {}

pub struct Encounter {
    pub coordinate: Coordinate,
}

impl Encounter {
    pub fn is_playable_event(&self, castle: &Castle) -> bool {
        match self.tile(castle) {
            Some(Tile::Door(EventID::FairyEvent(_)))
            | Some(Tile::Door(EventID::GenieEvent(_)))
            | Some(Tile::Door(EventID::MonsterEvent(_))) => true,
            _ => false,
        }
    }

    pub fn tile<'a>(&self, castle: &'a Castle) -> Option<&'a Tile> {
        castle.get_object(self.coordinate)
    }

    pub fn reset(&mut self, coordinate: Coordinate) {
        self.coordinate = coordinate;
    }
}

pub struct Player {
    pub hp: i32,
    pub mana: i32,
    pub money: i32,
    pub attack_power: (i32, i32),
    pub current_coordinate: Coordinate,
    pub inventory: HashMap<Item, i32>,
    pub status: PlayerStatus,
    pub accumulator: f32,
    pub encounter: Encounter,
}

impl Player {
    pub fn spawn(castle: &Castle) -> Self {
        let current_coordinate = Self::select_initial_location(castle, 0);
        let encounter = Encounter {
            coordinate: current_coordinate,
        };

        Self {
            hp: 100,
            mana: 100,
            money: 100,
            attack_power: (1, 5),
            current_coordinate,
            inventory: HashMap::new(),
            status: PlayerStatus::Roam,
            accumulator: 0.0,
            encounter,
        }
    }

    pub fn select_initial_location(castle: &Castle, floor: i32) -> Coordinate {
        let mut keys = filter_possible_coordinates(&castle.layout, floor, Tile::Floor);

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

        self.encounter.coordinate = new_coordinate;

        if matches!(
            self.encounter.tile(castle),
            Some(Tile::Floor) | Some(Tile::Door(EventID::Empty))
        ) {
            self.current_coordinate.x += player_direction.x;
            self.current_coordinate.y += player_direction.y;
        } else if self.encounter.tile(castle).is_none() {
            // Out of bounds, perform a wrap. Castle coordinated go from 0 to
            // max - 1, hence modulus should put max to 0 and -1 to max - 1
            // Only player allowed to wrap
            match direction {
                KeyCode::Left | KeyCode::Right => {
                    self.current_coordinate.x = new_coordinate.x.rem_euclid(castle.width);
                }
                KeyCode::Down | KeyCode::Up => {
                    self.current_coordinate.y = new_coordinate.y.rem_euclid(castle.depth);
                }
                _ => (),
            }
        }
    }

    pub fn in_inventory(&self) -> bool {
        self.status == PlayerStatus::Inventory
    }

    pub fn open_inventory(&mut self) {
        if matches!(Controller::get_key(), Some(KeyCode::I)) {
            self.update_status(PlayerStatus::Inventory);
        }
    }

    pub fn in_shop(&self) -> bool {
        self.status == PlayerStatus::Shop
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
    fn increment_floor(&mut self) -> &mut i32 {
        &mut self.current_coordinate.z
    }
}
