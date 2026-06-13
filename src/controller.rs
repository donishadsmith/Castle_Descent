use macroquad::input::{KeyCode, get_keys_down, get_keys_pressed};

use crate::castle::Castle;
use crate::player::{Player, PlayerStatus};
use crate::utils::prelude::GameState;

const PLAYER_DISPLACEMENT: f32 = 0.10;

fn player_keyboard(key_press: KeyCode, player: &mut Player, castle: &Castle) {
    if matches!(
        key_press,
        KeyCode::Left | KeyCode::Right | KeyCode::Up | KeyCode::Down
    ) {
        player.update_position(key_press, castle);
    }
}

pub struct Controller {}

impl Controller {
    pub fn roam(player: &mut Player, castle: &Castle, dt: &f32, game_state: &mut GameState) {
        if player.status != PlayerStatus::Roam {
            return;
        }

        let mut key_press = get_keys_pressed().iter().next().cloned();
        if key_press.is_none() {
            key_press = get_keys_down().iter().next().cloned();
        }

        if let Some(key) = key_press
            && player.status == PlayerStatus::Roam
        {
            // Only accumulate if a key is down else large skipping occurs
            player.accumulator += dt;

            while player.accumulator >= PLAYER_DISPLACEMENT {
                if matches!(player.status, PlayerStatus::Roam) {
                    player_keyboard(key, player, &castle);
                }

                player.accumulator -= PLAYER_DISPLACEMENT;
            }

            if matches!(key, KeyCode::Q | KeyCode::Escape) {
                *game_state = GameState::Quit;
            }

            if matches!(key, KeyCode::P) {
                *game_state = GameState::Paused;
            } else if key_press.is_some() && *game_state != GameState::Quit {
                *game_state = GameState::Active;
            }
        }
    }
}
