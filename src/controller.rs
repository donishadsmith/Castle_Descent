use macroquad::input::{KeyCode, get_keys_down, get_keys_pressed};

use crate::{
    castle::Castle,
    player::{Player, PlayerStatus},
    utils::prelude::{EntityStatus, GameState},
    zombie::{Zombie, ZombieStatus},
};

const PLAYER_DISPLACEMENT: f32 = 0.10;
const ZOMBIE_DISPLACEMENT: f32 = 0.90;

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
    pub fn roam(
        castle: &Castle,
        player: &mut Player,
        zombie: &mut Zombie,
        dt: &f32,
        game_state: &mut GameState,
    ) {
        if *game_state != GameState::Active {
            return;
        }

        if player.status != PlayerStatus::Roam {
            return;
        }

        if let Some(key) = Controller::get_key()
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
        }

        if matches!(game_state, GameState::Active) && matches!(player.status, PlayerStatus::Roam) {
            zombie.accumulator += dt;

            if zombie.accumulator >= ZOMBIE_DISPLACEMENT {
                zombie.update_status(ZombieStatus::Roam);
                zombie.chase_player(&player, &castle);

                zombie.accumulator = 0.0;
            }
        }
    }

    pub fn get_key() -> Option<KeyCode> {
        let mut key_press = get_keys_pressed().iter().next().cloned();
        if key_press.is_none() {
            key_press = get_keys_down().iter().next().cloned();
        }

        key_press
    }

    pub fn mutate_game_state(game_state: &mut GameState) {
        if let Some(key) = Self::get_key() {
            Self::quit(&key, game_state);
            Self::pause(&key, game_state);
        }
    }

    pub fn quit(key: &KeyCode, game_state: &mut GameState) {
        if matches!(*key, KeyCode::Q | KeyCode::Escape) {
            *game_state = GameState::Quit;
        }
    }

    pub fn pause(key: &KeyCode, game_state: &mut GameState) {
        if matches!(key, KeyCode::P) {
            *game_state = GameState::Paused;
        } else if *game_state != GameState::Quit {
            *game_state = GameState::Active;
        }
    }
}
