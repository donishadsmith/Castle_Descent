use macroquad::input::{KeyCode, get_keys_down, get_keys_pressed};

use crate::{
    castle::Castle,
    player::{Player, PlayerStatus},
    utils::prelude::{EntityStatus, GameState},
    zombie::{Zombie, ZombieStatus},
};

const PLAYER_DISPLACEMENT: f32 = 0.10;
//const ZOMBIE_DISPLACEMENT: f32 = 0.90;

const ZOMBIE_DISPLACEMENT: f32 = 10e37;

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

        if !matches!(player.status, PlayerStatus::Roam | PlayerStatus::Hide) {
            return;
        }

        let was_hidden = player.status == PlayerStatus::Hide;
        let key = if was_hidden {
            Controller::get_press()
        } else {
            Controller::get_key()
        };

        if let Some(key) = key {
            let start_coordinate = player.current_coordinate;

            if was_hidden {
                player.update_status(PlayerStatus::Roam);
                player_keyboard(key, player, castle);
                player.accumulator = 0.0;

                if player.current_coordinate == start_coordinate {
                    player.update_status(PlayerStatus::Hide);
                }
            } else {
                player.accumulator += dt;
                while player.accumulator >= PLAYER_DISPLACEMENT {
                    player_keyboard(key, player, castle);
                    player.accumulator -= PLAYER_DISPLACEMENT;
                }
            }
        }
        if matches!(game_state, GameState::Active)
            && matches!(player.status, PlayerStatus::Roam | PlayerStatus::Hide)
            && !player.effects.freeze_zombie()
        {
            zombie.accumulator += dt;

            if zombie.accumulator >= ZOMBIE_DISPLACEMENT {
                zombie.update_status(ZombieStatus::Roam);

                if player.status == PlayerStatus::Hide {
                    zombie.wander(castle);
                } else {
                    zombie.chase_player(player, castle);
                }

                zombie.accumulator = 0.0;
            }
        }
    }

    pub fn shop(player: &mut Player) {
        Controller::escape(player);
    }

    pub fn inventory(player: &mut Player) {
        Controller::escape(player)
    }

    pub fn get_key() -> Option<KeyCode> {
        let mut key_press = get_keys_pressed().iter().next().cloned();
        if key_press.is_none() {
            key_press = get_keys_down().iter().next().cloned();
        }

        key_press
    }

    pub fn get_press() -> Option<KeyCode> {
        get_keys_pressed().iter().next().cloned()
    }

    pub fn mutate_game_state(game_state: &mut GameState) {
        if let Some(key) = Self::get_key() {
            Self::quit(&key, game_state);
            Self::pause(&key, game_state);
            Self::resume(&key, game_state);
        }
    }

    pub fn quit(key: &KeyCode, game_state: &mut GameState) {
        if matches!(*key, KeyCode::Q) {
            *game_state = GameState::Quit;
        }
    }

    fn pause(key: &KeyCode, game_state: &mut GameState) {
        if matches!(key, KeyCode::P) {
            *game_state = GameState::Paused;
        }
    }

    fn resume(key: &KeyCode, game_state: &mut GameState) {
        if *game_state == GameState::Paused && matches!(key, KeyCode::Escape) {
            *game_state = GameState::Active;
        }
    }

    fn escape(player: &mut Player) {
        if matches!(Controller::get_key(), Some(KeyCode::Escape)) {
            player.update_status(PlayerStatus::Roam);
            player.encounter.coordinate = player.current_coordinate;
        }
    }
}
