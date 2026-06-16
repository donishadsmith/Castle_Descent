use std::collections::HashMap;

use macroquad::prelude::*;
use strum::{EnumCount, EnumIter, IntoEnumIterator};

use crate::player::Player;

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
    pub fn display_shop(
        player: &Player,
        texture_map: &HashMap<&str, Texture2D>,
        scale_params: DrawTextureParams,
    ) {
        let screen_x = screen_width() / 2.0;
        let screen_y = screen_height() / 2.0;

        let mut increment_x = Merchant::max_space_distance();
        for item in Item::iter() {
            draw_texture_ex(
                texture_map.get(item.identity()).unwrap(),
                screen_x + increment_x,
                screen_y,
                WHITE,
                scale_params.clone(),
            );

            increment_x += Merchant::x_increment();
        }

        draw_text(
            format!("Money: {}", player.money),
            screen_x - 80.0,
            screen_y + 90.0,
            30.0,
            WHITE,
        );

        draw_text(
            "Navigate (Up/Down arrows) | Exit (Esc) | Select (Enter)".to_string(),
            screen_x - 260.0,
            screen_y + 120.0,
            20.0,
            WHITE,
        );
    }

    pub fn max_space_distance() -> f32 {
        -30.0 * (Item::iter().count() as f32)
    }

    pub fn x_increment() -> f32 {
        60.0
    }
}
