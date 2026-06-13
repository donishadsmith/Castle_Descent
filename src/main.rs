use macroquad::prelude::*;

use Castle_Descent::{
    castle::{Castle, EventID, Tile},
    player::{Player, PlayerStatus},
    utils::prelude::*,
    zombie::{Zombie, ZombieStatus},
};

const TILE_SIZE: f32 = 32.0;
const PLAYER_DISPLACEMENT: f32 = 0.10;
const ZOMBIE_DISPLACEMENT: f32 = 0.90;

#[derive(PartialEq)]
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

fn draw_asset(asset: &Texture2D, coordinate: Coordinate, scale_params: DrawTextureParams) {
    draw_texture_ex(
        asset,
        coordinate.to_float(Component::X) * TILE_SIZE,
        coordinate.to_float(Component::Y) * TILE_SIZE,
        WHITE,
        scale_params,
    );
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
    player.current_coordinate == zombie.current_coordinate
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

    let door_bytes: &[u8] = include_bytes!("../assets/door.png");
    let merchant_bytes: &[u8] = include_bytes!("../assets/merchant.png");
    let monster_bytes: &[u8] = include_bytes!("../assets/monster.png");
    let fairy_bytes: &[u8] = include_bytes!("../assets/fairy.png");
    let genie_bytes: &[u8] = include_bytes!("../assets/genie.png");
    let player_bytes: &[u8] = include_bytes!("../assets/player.png");
    let zombie_bytes: &[u8] = include_bytes!("../assets/zombie.png");

    let door_texture = Texture2D::from_file_with_format(door_bytes, None);
    let merchant_texture = Texture2D::from_file_with_format(merchant_bytes, None);
    let monster_texture = Texture2D::from_file_with_format(monster_bytes, None);
    let fairy_texture = Texture2D::from_file_with_format(fairy_bytes, None);
    let genie_texture = Texture2D::from_file_with_format(genie_bytes, None);
    let player_texture = Texture2D::from_file_with_format(player_bytes, None);
    let zombie_texture = Texture2D::from_file_with_format(zombie_bytes, None);

    let mut player_accumulator: f32 = 0.0;
    let mut zombie_accumulator: f32 = 0.0;

    let scale_params = DrawTextureParams {
        dest_size: Some(vec2(TILE_SIZE, TILE_SIZE)),
        ..Default::default()
    };

    let mut game_state = GameState::Active;

    let final_game_state = loop {
        let dt = get_frame_time().min(0.25);

        clear_background(BLACK);

        for (coordinate, tile) in &castle.layout {
            if coordinate.z == castle.current_floor {
                match tile {
                    Tile::Floor => {
                        draw_rectangle(
                            coordinate.to_float(Component::X) * TILE_SIZE,
                            coordinate.to_float(Component::Y) * TILE_SIZE,
                            TILE_SIZE,
                            TILE_SIZE,
                            BLACK,
                        );
                    }
                    Tile::Merchant => {
                        draw_asset(&merchant_texture, *coordinate, scale_params.clone());
                    }
                    Tile::Door(_) => {
                        let is_active_event = *coordinate == player.intended_coordinate
                            && matches!(player.status, PlayerStatus::Event);

                        if !is_active_event {
                            draw_asset(&door_texture, *coordinate, scale_params.clone());
                        }
                    }
                }
            }
        }

        if !matches!(player.status, PlayerStatus::Hide) {
            draw_asset(
                &player_texture,
                player.current_coordinate,
                scale_params.clone(),
            );
        }

        draw_asset(
            &zombie_texture,
            zombie.current_coordinate,
            scale_params.clone(),
        );

        // https://gamedev.stackexchange.com/questions/187660/fixed-timestep-game-loop-why-interpolation
        let mut key_press = get_keys_pressed().iter().next().cloned();
        if key_press.is_none() {
            key_press = get_keys_down().iter().next().cloned();
        }

        if let Some(key) = key_press {
            // Only accumulate if a key is down else large skipping occurs
            player_accumulator += dt;

            while player_accumulator >= PLAYER_DISPLACEMENT {
                if matches!(player.status, PlayerStatus::Roam) {
                    player_keyboard(key, &mut player, &castle);
                }

                player_accumulator -= PLAYER_DISPLACEMENT;
            }

            if matches!(key, KeyCode::Q | KeyCode::Escape) {
                break;
            }

            if matches!(key, KeyCode::P) {
                game_state = GameState::Paused;
            } else if key_press.is_some() {
                game_state = GameState::Active;
            }
        }

        // TODO: Update game logic to play event but not get stuck
        // on a conditional that ends up being always true
        let on_event_tile = castle
            .get_object(player.intended_coordinate)
            .is_some_and(|tile| !matches!(tile, Tile::Floor));
        if on_event_tile {
            let tile = *castle
                .get_mutable_object(player.intended_coordinate)
                .unwrap();

            let is_playable = matches!(
                tile,
                Tile::Door(EventID::MonsterEvent(_))
                    | Tile::Door(EventID::FairyEvent(_))
                    | Tile::Door(EventID::GenieEvent(_))
            );
            if is_playable {
                match tile {
                    Tile::Door(EventID::MonsterEvent(_)) => draw_asset(
                        &monster_texture,
                        player.intended_coordinate,
                        scale_params.clone(),
                    ),
                    Tile::Door(EventID::FairyEvent(_)) => draw_asset(
                        &fairy_texture,
                        player.intended_coordinate,
                        scale_params.clone(),
                    ),
                    Tile::Door(EventID::GenieEvent(_)) => draw_asset(
                        &genie_texture,
                        player.intended_coordinate,
                        scale_params.clone(),
                    ),
                    _ => (),
                };
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
            zombie.update_status(ZombieStatus::Frozen)
        }

        if matches!(game_state, GameState::Active) && matches!(player.status, PlayerStatus::Roam) {
            zombie_accumulator += dt;

            if zombie_accumulator >= ZOMBIE_DISPLACEMENT {
                zombie.update_status(ZombieStatus::Roam);
                zombie.chase_player(&player, &castle);

                zombie_accumulator = 0.0;
            }
        }

        next_frame().await
    };
}
