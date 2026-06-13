use macroquad::prelude::*;

use CastleDescent::{
    castle::{Castle, Tile},
    controller::Controller,
    events::prelude::EventID,
    player::{
        Player,
        PlayerStatus::{self, Event},
    },
    utils::prelude::*,
    zombie::{Zombie, ZombieStatus},
};

const TILE_SIZE: f32 = 32.0;
const ZOMBIE_DISPLACEMENT: f32 = 0.90;

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
            GameState::Quit => GameState::Quit,
            _ => GameState::Active,
        }
    }
}

fn draw_transparant_screen(
    first_text: &str,
    second_text: &str,
    first_x_shift: f32,
    second_x_shift: f32,
    first_y_shift: f32,
    second_y_shift: f32,
) {
    draw_rectangle(
        0.0,
        0.0,
        screen_width(),
        screen_height(),
        Color::new(0.0, 0.0, 0.0, 0.7),
    );
    draw_text(
        first_text,
        screen_width() / 2.0 - first_x_shift,
        screen_height() / 2.0 + first_y_shift,
        30.0,
        WHITE,
    );
    draw_text(
        second_text,
        screen_width() / 2.0 - second_x_shift,
        screen_height() / 2.0 + second_y_shift,
        20.0,
        WHITE,
    );
}

fn reset_game(game_state: &mut GameState) -> bool {
    if matches!(game_state, GameState::Win | GameState::Lose) {
        let text_str = if *game_state == GameState::Win {
            "You Won!"
        } else {
            "You Lost."
        };

        draw_transparant_screen(
            &text_str,
            &"Press 'r' to restart or 'q' to quit.",
            80.0,
            160.0,
            -50.0,
            -10.0,
        )
    }

    let Some(key) = Controller::get_key() else {
        return false;
    };

    if key == KeyCode::R && matches!(game_state, GameState::Win | GameState::Lose) {
        *game_state = GameState::Active;
        true
    } else {
        Controller::quit(&key, game_state);
        false
    }
}

#[macroquad::main("Castle Descent")]
async fn main() {
    //set_fullscreen(true);

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

    let scale_params = DrawTextureParams {
        dest_size: Some(vec2(TILE_SIZE, TILE_SIZE)),
        ..Default::default()
    };

    let mut game_state = GameState::Active;

    loop {
        clear_background(BLACK);
        if game_state == GameState::Quit {
            break;
        }

        let dt = get_frame_time().min(0.25);
        for (coordinate, tile) in &castle.layout {
            if coordinate.z != castle.current_floor {
                continue;
            }

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
                Tile::Shop(_) => {
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

        Controller::roam(&mut player, &castle, &dt, &mut game_state);

        if let Some(tile) = castle.get_mutable_object(player.intended_coordinate) {
            match tile {
                Tile::Door(event @ EventID::MonsterEvent(_)) => {
                    draw_asset(
                        &monster_texture,
                        player.intended_coordinate,
                        scale_params.clone(),
                    );

                    event.activate(&mut player, &mut zombie, &mut game_state)
                }
                Tile::Door(event @ EventID::FairyEvent(_)) => {
                    draw_asset(
                        &fairy_texture,
                        player.intended_coordinate,
                        scale_params.clone(),
                    );

                    event.activate(&mut player, &mut zombie, &mut game_state)
                }
                Tile::Door(event @ EventID::GenieEvent(_)) => {
                    draw_asset(
                        &genie_texture,
                        player.intended_coordinate,
                        scale_params.clone(),
                    );

                    event.activate(&mut player, &mut zombie, &mut game_state)
                }
                Tile::Shop(merchant @ _) => {}
                Tile::Door(event @ EventID::Empty) => {
                    event.activate(&mut player, &mut zombie, &mut game_state)
                }
                Tile::Door(event @ EventID::Exit) => {
                    event.activate(&mut player, &mut zombie, &mut game_state)
                }
                _ => (),
            };
        }

        if matches!(game_state, GameState::Active) && matches!(player.status, PlayerStatus::Roam) {
            zombie.accumulator += dt;

            if zombie.accumulator >= ZOMBIE_DISPLACEMENT {
                zombie.update_status(ZombieStatus::Roam);
                zombie.chase_player(&player, &castle);

                zombie.accumulator = 0.0;
            }
        }

        Controller::mutate_game_state(&mut game_state);
        if !matches!(game_state, GameState::Win | GameState::Lose) {
            game_state = check_game_status(&castle, &player, &zombie, game_state);
        }

        if matches!(game_state, GameState::Paused) {
            draw_transparant_screen(
                &"Game Paused",
                &"Press any key to continue.",
                100.0,
                140.0,
                -50.0,
                -10.0,
            );

            zombie.update_status(ZombieStatus::Frozen)
        }

        if reset_game(&mut game_state) {
            clear_background(BLACK);
            (castle, player, zombie) = initialize();
        }

        next_frame().await
    }
}
