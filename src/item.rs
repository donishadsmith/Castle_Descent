use macroquad::prelude::*;
use strum::{EnumCount, EnumIter, IntoEnumIterator};

#[derive(Copy, Clone, Hash, EnumCount, Eq, EnumIter, PartialEq)]
pub enum Item {
    CrystalBall,
    Hourglass,
    Meat,
    Potion,
}

impl Item {
    pub fn description(&self) -> &'static str {
        match self {
            Item::CrystalBall => "Crystal Ball: Reveals the exit.",
            Item::Hourglass => "Hourglass: Freezes zombie temporarily (5-10 seconds).",
            Item::Meat => "Meat: Heals 20 hp.",
            Item::Potion => "Potion: Restores 20 mana.",
        }
    }

    pub fn cost(&self) -> i32 {
        match self {
            Item::Meat => 10,
            _ => 100,
        }
    }

    pub fn identity_to_str(&self) -> &str {
        match self {
            Item::CrystalBall => "crystal_ball",
            Item::Hourglass => "hourglass",
            Item::Meat => "meat",
            Item::Potion => "potion",
        }
    }

    pub fn all() -> Vec<Item> {
        Item::iter().collect()
    }

    pub fn max_distance() -> f32 {
        -60.0 * (Item::all().len() as f32)
    }

    pub fn item_and_price() -> Vec<(Item, i32)> {
        Item::all()
            .into_iter()
            .map(|item| (item, item.cost()))
            .collect()
    }
}
