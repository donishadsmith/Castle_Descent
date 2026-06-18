use std::collections::{HashMap, hash_map::Entry::Vacant};

use macroquad::prelude::*;

use crate::{
    castle::{Castle, Tile},
    controller::Controller,
    events::EventID,
    item::Item,
    menu::{Menu, MenuType},
    utils::prelude::*,
    zombie::Zombie,
};

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
        matches!(
            self.tile(castle),
            Some(Tile::Door(EventID::FairyEvent(_)))
                | Some(Tile::Door(EventID::GenieEvent(_)))
                | Some(Tile::Door(EventID::MonsterEvent(_)))
        )
    }

    pub fn tile<'a>(&self, castle: &'a Castle) -> Option<&'a Tile> {
        castle.get_ref_object(self.coordinate)
    }
}

pub struct Inventory {
    storage: HashMap<Item, i32>,
}

impl Inventory {
    fn new() -> Self {
        Self {
            storage: HashMap::new(),
        }
    }

    pub fn count(&self, item: Item) -> i32 {
        *self.storage.get(&item).unwrap_or(&0)
    }

    pub fn storage_to_pairs(&self) -> Vec<(Item, i32)> {
        self.storage.iter().map(|(item, n)| (*item, *n)).collect()
    }

    fn delete_empty(&mut self) {
        self.storage.retain(|_, value| *value > 0);
    }

    pub fn decrement_item(&mut self, item: Item) {
        if let Some(n) = self.storage.get_mut(&item) {
            *n -= 1
        }

        self.delete_empty();
    }

    pub fn add_item(&mut self, item: Item, quantity: i32) {
        if quantity == 0 {
            return;
        }

        /*
        warning: usage of `contains_key` followed by `insert` on a `HashMap`
        if !self.storage.contains_key(&item) {
            self.storage.insert(item, quantity);
        } else {
            if let Some(n) = self.storage.get_mut(&item) {
                *n += quantity;
            }*/

        // clippy recommendation
        if let Vacant(e) = self.storage.entry(item) {
            e.insert(quantity);
        } else {
            if let Some(n) = self.storage.get_mut(&item) {
                *n += quantity;
            }
        }
    }

    pub fn max_space_distance(&self) -> f32 {
        -60.0 * (self.storage.len() as f32)
    }
}

pub struct Effects {
    active: Vec<Item>,
}

impl Effects {
    fn new() -> Self {
        Self { active: Vec::new() }
    }

    pub fn inactivate(&mut self, item: Item) {
        self.active.retain(|key| *key != item);
    }

    pub fn reveal_exit(&self) -> bool {
        self.in_effect(&Item::CrystalBall)
    }

    pub fn freeze_zombie(&self) -> bool {
        self.in_effect(&Item::Hourglass)
    }

    pub fn freeze_time(&self) -> f32 {
        choose_random_range(5..11) as f32
    }

    pub fn in_effect(&self, item: &Item) -> bool {
        self.active.contains(item)
    }

    pub fn add(&mut self, item: Item) {
        self.active.push(item);
    }

    pub fn any_active(&self) -> bool {
        !self.active.is_empty()
    }

    pub fn count(&self, item: Item) -> usize {
        self.active.iter().filter(|&&x| x == item).count()
    }
}

pub struct Player {
    pub hp: i32,
    pub mana: i32,
    pub money: i32,
    pub attack_power: (i32, i32),
    pub current_coordinate: Coordinate,
    pub inventory: Inventory,
    pub menu: Option<Menu>,
    pub status: PlayerStatus,
    pub accumulator: f32,
    pub encounter: Encounter,
    pub effects: Effects,
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
            inventory: Inventory::new(),
            menu: None,
            status: PlayerStatus::Roam,
            accumulator: 0.0,
            encounter,
            effects: Effects::new(),
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

    pub fn replenish_stats(&mut self) {
        // Can technically select more than needed since stats cap
        // at 100, deal with that later
        if self.effects.in_effect(&Item::Meat) {
            for _ in 0..self.effects.count(Item::Meat) {
                while self.hp < 100 {
                    self.hp += 20
                }

                if self.hp > 100 {
                    break;
                }
            }

            self.cap_stat("hp");
        }

        if self.effects.in_effect(&Item::Potion) {
            for _ in 0..self.effects.count(Item::Potion) {
                while self.mana < 100 {
                    self.mana += 20
                }

                if self.mana > 100 {
                    break;
                }
            }

            self.cap_stat("mana");
        }
    }

    fn cap_stat(&mut self, stat: &str) {
        match stat {
            "hp" => {
                self.hp = if self.hp > 100 { 100 } else { self.hp };
            }
            _ => {
                self.mana = if self.hp > 100 { 100 } else { self.mana };
            }
        }
    }

    pub fn in_shop(&self) -> bool {
        self.status == PlayerStatus::Shop
    }

    pub fn in_event(&self) -> bool {
        self.status == PlayerStatus::Event
    }

    pub fn dead(&mut self) {
        if self.hp <= 0 {
            self.update_status(PlayerStatus::Lose);
        }
    }

    pub fn caught(&mut self, zombie: &Zombie) {
        if self.current_coordinate == zombie.current_coordinate {
            self.update_status(PlayerStatus::Lose)
        }
    }

    pub fn buy(&mut self, item: Item, quantity: i32) {
        self.inventory.add_item(item, quantity);
        self.money = (self.money - item.cost() * quantity).max(0);
    }

    pub fn item_limit(&self, item: Item, menu_type: &MenuType) -> i32 {
        match menu_type {
            // Both are i32; gives truncated integer
            MenuType::Shop => self.money / item.cost(),
            MenuType::Inventory => {
                if matches!(item, Item::Meat | Item::Potion) {
                    self.compute_stat_limit(item)
                } else {
                    self.inventory.count(item)
                }
            }
        }
    }

    fn compute_stat_limit(&self, item: Item) -> i32 {
        let restore_points = 20.0;
        // Apparantly div_ceil is an library feature?
        let cap_amount = |x: i32| ((100.0 - x as f32) / restore_points as f32).ceil();
        match item {
            Item::Meat => cap_amount(self.hp) as i32,
            _ => cap_amount(self.mana) as i32,
        }
    }

    pub fn use_item(&mut self, item: Item, quantity: i32) {
        let available = self.inventory.count(item).min(quantity);
        for _ in 0..available {
            self.inventory.decrement_item(item);
            self.effects.add(item);
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
