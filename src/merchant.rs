use crate::player::Player;

#[derive(Hash, Eq, PartialEq)]
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

    pub fn identity(&self) -> &str {
        match self {
            Item::CrystalBall => "crystal_ball",
            Item::Hourglass => "hourglass",
            Item::Meat => "meat",
            Item::Potion => "potion",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Merchant {}

impl Merchant {
    pub fn shop(player: &mut Player) {}
}
