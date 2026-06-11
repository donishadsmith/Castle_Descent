use macroquad::prelude::*;

use Castle_Descent::{
    castle::{Castle, Reveal, Tile},
    player::{Player, PlayerStatus},
    utils::{Descent, filter_possible_coordinates, get_direction},
    zombie::{Zombie, ZombieStatus},
};

const TILE_SIZE: f32 = 32.0;
const PLAYER_SPEED: f32 = 0.10;
const ZOMBIE_SPEED: f32 = 0.90;

enum GameState {
    Win,
    Lose,
    Paused,
    Active,
}

// Will be used as an initializer of a few things
fn initialize() -> (Castle, Player, Zombie) {
    let castle = Castle::generate();
    let player = Player::spawn(&castle);
    let zombie = Zombie::spawn(&castle, &player);

    (castle, player, zombie)
}

fn player_keyboard(key_press: KeyCode, player: &mut Player, castle: &Castle) {
    if matches!(
        key_press,
        KeyCode::Left | KeyCode::Right | KeyCode::Up | KeyCode::Down
    ) {
        player.update_position(key_press, castle);
    }
}

fn player_caught(player: &Player, zombie: &Zombie) -> bool {
    player.current_position == zombie.current_position
}

fn reached_final_exit(castle: &Castle, player: &Player) -> bool {
    if player.current_position.2 != castle.max_floors() {
        false
    } else {
        // There will always be one exit
        let exit_coordinate = filter_possible_coordinates(
            &castle.layout,
            castle.max_floors(),
            Tile::Door(Reveal::Exit),
        )[0];
        if (exit_coordinate.0 - player.current_position.0) == 0
            && (exit_coordinate.1 - player.current_position.1) == 0
        {
            true
        } else {
            false
        }
    }
}

fn player_dead(player: &Player) -> bool {
    player.hp <= 0
}

fn next_level<Entity: Descent>(mut object: Entity) {
    object.descend();
}

fn check_game_status(
    castle: &Castle,
    player: &Player,
    zombie: &Zombie,
    game_status: GameState,
) -> GameState {
    if player_caught(&player, &zombie) || player_dead(&player) {
        GameState::Lose
    } else if reached_final_exit(&castle, &player) {
        GameState::Win
    } else {
        match game_status {
            GameState::Paused => GameState::Paused,
            _ => GameState::Active,
        }
    }
}

#[macroquad::main("Castle Descent")]
async fn main() {
    let (mut castle, mut player, mut zombie) = initialize();

    let door_texture = load_texture("assets/door.png").await.unwrap();
    let merchant_texture = load_texture("assets/merchant.png").await.unwrap();
    let monster_texture = load_texture("assets/monster.png").await.unwrap();
    let fairy_texture = load_texture("assets/fairy.png").await.unwrap();
    let genie_texture = load_texture("assets/genie.png").await.unwrap();
    let player_texture = load_texture("assets/player.png").await.unwrap();
    let zombie_texture = load_texture("assets/zombie.png").await.unwrap();

    let mut player_accumulator: f32 = 0.0;
    let mut zombie_accumulator: f32 = 0.0;

    let scale_params = DrawTextureParams {
        dest_size: Some(vec2(TILE_SIZE, TILE_SIZE)),
        ..Default::default()
    };

    let mut game_state = GameState::Active;

    let final_game_state = loop {
        clear_background(BLACK);

        for ((grid_x, grid_y, grid_z), tile) in &castle.layout {
            if *grid_z == castle.current_floor {
                let screen_x = *grid_x as f32 * TILE_SIZE;
                let screen_y = *grid_y as f32 * TILE_SIZE;

                match tile {
                    Tile::Floor => {
                        draw_rectangle(screen_x, screen_y, TILE_SIZE, TILE_SIZE, BLACK);
                    }
                    Tile::Merchant => {
                        draw_texture_ex(
                            &merchant_texture,
                            screen_x,
                            screen_y,
                            WHITE,
                            scale_params.clone(),
                        );
                    }
                    Tile::Door(reveal) => {
                        draw_texture_ex(
                            &door_texture,
                            screen_x,
                            screen_y,
                            WHITE,
                            scale_params.clone(),
                        );
                    }
                }
            }
        }

        draw_texture_ex(
            &player_texture,
            player.current_position.0 as f32 * TILE_SIZE,
            player.current_position.1 as f32 * TILE_SIZE,
            WHITE,
            scale_params.clone(),
        );
        draw_texture_ex(
            &zombie_texture,
            zombie.current_position.0 as f32 * TILE_SIZE,
            zombie.current_position.1 as f32 * TILE_SIZE,
            WHITE,
            scale_params.clone(),
        );

        // https://gamedev.stackexchange.com/questions/187660/fixed-timestep-game-loop-why-interpolation
        // But no integration
        // //https://www.gamedeveloper.com/programming/movement-prediction
        let current_speed = get_frame_time();
        let mut predicted_player_position = player.current_position;
        let mut key_press = get_keys_pressed().iter().next().cloned();
        if !matches!(key_press, Some(KeyCode)) {
            key_press = get_keys_down().iter().next().cloned();
        }

        if matches!(key_press, Some(KeyCode)) {
            // Only accumulate if a key is down else large skipping occurs
            player_accumulator += get_frame_time();

            while player_accumulator >= PLAYER_SPEED {
                if matches!(key_press, Some(KeyCode)) && matches!(player.status, PlayerStatus::Roam)
                {
                    player_keyboard(key_press.unwrap(), &mut player, &castle);
                }

                player_accumulator -= PLAYER_SPEED;
            }

            let direction = get_direction(key_press.unwrap());
            let velocity = (
                direction.0 * (current_speed as i8),
                direction.1 * (current_speed as i8),
            );
            predicted_player_position = (
                player.current_position.0 + velocity.0,
                player.current_position.1 + velocity.1,
                player.current_position.2,
            );

            if matches!(key_press.unwrap(), KeyCode::Q | KeyCode::Escape) {
                break;
            }

            if matches!(key_press.unwrap(), KeyCode::P) {
                game_state = GameState::Paused;
            } else if !key_press.is_none() {
                game_state = GameState::Active;
            }
        }

        if matches!(game_state, GameState::Paused) {
            draw_rectangle(
                0.0,
                0.0,
                screen_width(),
                screen_height(),
                Color::new(0.0, 0.0, 0.0, 0.7),
            );
            draw_text(
                "Game Paused",
                screen_width() / 2.0 - 100.0,
                screen_height() / 2.0,
                30.0,
                WHITE,
            );
            draw_text(
                "Press any key to continue",
                screen_width() / 2.0 - 140.0,
                screen_height() / 2.0 + 40.0,
                20.0,
                WHITE,
            );
            zombie.change_status(ZombieStatus::Frozen)
        }

        if matches!(game_state, GameState::Active) && matches!(player.status, PlayerStatus::Roam) {
            zombie_accumulator += get_frame_time();

            while zombie_accumulator >= ZOMBIE_SPEED {
                zombie.change_status(ZombieStatus::Roam);
                zombie.chase_player(&player, predicted_player_position, &castle);

                zombie_accumulator -= ZOMBIE_SPEED;
            }
        }

        next_frame().await
    };
}
