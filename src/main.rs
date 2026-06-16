/*
TODO:
- Implement merchant
- Add logic for Playable events (Monster, Fairy, Genie), includes the menu for each.
- Add logic for inventory display and selection
*/

// PNG assets from https://emoji.aranja.com/
// if needed later, go back to having a library + binary crate

mod castle;
mod controller;
mod events;
mod merchant;
mod player;
mod utils;
mod zombie;

use ::std::collections::HashMap;
use macroquad::prelude::*;

use crate::{
    castle::{Castle, Tile},
    controller::Controller,
    events::EventID,
    merchant::{Item, Merchant},
    player::{Player, PlayerStatus},
    utils::prelude::*,
    zombie::{Zombie, ZombieStatus},
};

const TILE_SIZE: f32 = 32.0;

struct Transition {
    remaining: f32,
    first_text: String,
    second_text: String,
}

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

fn render_castle(
    castle: &Castle,
    player: &Player,
    zombie: &Zombie,
    texture_map: &HashMap<&str, Texture2D>,
    scale_params: &DrawTextureParams,
) {
    for (coordinate, tile) in &castle.layout {
        if coordinate.z != castle.current_floor {
            continue;
        }

        if player.in_inventory() || player.in_shop() {
            return;
        }

        if player.status != PlayerStatus::Hide {
            draw_asset(
                texture_map.get("player").unwrap(),
                player.current_coordinate,
                scale_params.clone(),
            );
        }

        // Only render player an object on screen if in playable event
        if player.encounter.is_playable_event(castle) {
            return;
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
                draw_asset(
                    texture_map.get("merchant").unwrap(),
                    *coordinate,
                    scale_params.clone(),
                );
            }

            Tile::Door(_) => {
                let door_type = if player.effects.reveal_exit()
                    && matches!(
                        castle.get_ref_object(*coordinate),
                        Some(Tile::Door(EventID::Exit))
                    ) {
                    "exit"
                } else {
                    "door"
                };

                draw_asset(
                    texture_map.get(door_type).unwrap(),
                    *coordinate,
                    scale_params.clone(),
                );
            }
        }
    }

    draw_asset(
        texture_map.get("zombie").unwrap(),
        zombie.current_coordinate,
        scale_params.clone(),
    );

    if player.effects.freeze_zombie() {
        draw_asset(
            texture_map.get("x").unwrap(),
            zombie.current_coordinate,
            scale_params.clone(),
        );
    }
}

fn activate_event(
    castle: &mut Castle,
    player: &mut Player,
    zombie: &mut Zombie,
    texture_map: &HashMap<&str, Texture2D>,
    scale_params: &DrawTextureParams,
    game_state: &mut GameState,
    transition: &mut Option<Transition>,
) {
    if let Some(tile) = castle.get_mutable_object(player.encounter.coordinate) {
        match tile {
            Tile::Door(event @ EventID::MonsterEvent(_)) => {
                draw_asset(
                    texture_map.get("monster").unwrap(),
                    player.encounter.coordinate,
                    scale_params.clone(),
                );

                event.activate(player, game_state);
                event.replace_if_complete();
            }
            Tile::Door(event @ EventID::FairyEvent(_)) => {
                draw_asset(
                    texture_map.get("fairy").unwrap(),
                    player.encounter.coordinate,
                    scale_params.clone(),
                );

                event.activate(player, game_state);
                event.replace_if_complete();
            }
            Tile::Door(event @ EventID::GenieEvent(_)) => {
                draw_asset(
                    texture_map.get("genie").unwrap(),
                    player.encounter.coordinate,
                    scale_params.clone(),
                );

                event.activate(player, game_state);
                event.replace_if_complete();
            }
            Tile::Shop(_) => {
                if player.status != PlayerStatus::Shop {
                    player.update_status(PlayerStatus::Shop);
                }

                Merchant::display_shop(&player, &texture_map, scale_params.clone())
            }
            Tile::Door(EventID::Empty) => {
                if player.status != PlayerStatus::Hide {
                    player.update_status(PlayerStatus::Hide);
                }
            }
            Tile::Door(EventID::Exit) => {
                if player.current_coordinate.z < castle.max_floors() {
                    castle.increment_floor();
                    player.current_coordinate =
                        Player::select_initial_location(castle, castle.current_floor);
                    player.encounter.coordinate = player.current_coordinate;
                    player.effects.inactivate(Item::CrystalBall);

                    zombie.current_coordinate =
                        Zombie::select_initial_location(castle, player, castle.current_floor);
                    player.update_status(PlayerStatus::Roam);

                    *transition = Some(Transition {
                        remaining: 2.0,
                        first_text: "Found the exit.".to_string(),
                        second_text: format!("Moving to floor: {}", castle.current_floor),
                    });
                } else {
                    player.update_status(PlayerStatus::Win);
                }
            }
            Tile::Floor => {
                if !player.in_inventory() {
                    player.update_status(PlayerStatus::Roam)
                }
            }
        };
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

fn check_game_status(player: &Player, game_status: GameState) -> GameState {
    if player.status == PlayerStatus::Lose {
        GameState::Lose
    } else if player.status == PlayerStatus::Win {
        GameState::Win
    } else {
        match game_status {
            GameState::Paused => GameState::Paused,
            GameState::Quit => GameState::Quit,
            _ => GameState::Active,
        }
    }
}

fn reset_game(game_state: &mut GameState) -> bool {
    if matches!(game_state, GameState::Win | GameState::Lose) {
        let text_str = if *game_state == GameState::Win {
            "You Won!"
        } else {
            "You Lost."
        };

        draw_transparant_screen(
            text_str,
            "Press 'r' to restart or 'q' to quit.",
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
    let (mut castle, mut player, mut zombie) = initialize();

    let door_bytes: &[u8] = include_bytes!("../assets/door.png");
    let merchant_bytes: &[u8] = include_bytes!("../assets/merchant.png");
    let monster_bytes: &[u8] = include_bytes!("../assets/monster.png");
    let fairy_bytes: &[u8] = include_bytes!("../assets/fairy.png");
    let genie_bytes: &[u8] = include_bytes!("../assets/genie.png");
    let player_bytes: &[u8] = include_bytes!("../assets/player.png");
    let zombie_bytes: &[u8] = include_bytes!("../assets/zombie.png");
    let exit_bytes: &[u8] = include_bytes!("../assets/exit.png");
    let crystal_ball_bytes: &[u8] = include_bytes!("../assets/crystal_ball.png");
    let meat_bytes: &[u8] = include_bytes!("../assets/meat.png");
    let hourglass_bytes: &[u8] = include_bytes!("../assets/hourglass.png");
    let potion_bytes: &[u8] = include_bytes!("../assets/potion.png");
    let x_bytes: &[u8] = include_bytes!("../assets/x.png");

    let texture_map: HashMap<&str, Texture2D> = hashmap!(
        "door" ; Texture2D::from_file_with_format(door_bytes, None),
        "merchant" ; Texture2D::from_file_with_format(merchant_bytes, None),
        "monster" ; Texture2D::from_file_with_format(monster_bytes, None),
        "fairy" ; Texture2D::from_file_with_format(fairy_bytes, None),
        "genie" ; Texture2D::from_file_with_format(genie_bytes, None),
        "player" ; Texture2D::from_file_with_format(player_bytes, None),
        "zombie" ; Texture2D::from_file_with_format(zombie_bytes, None),
        "exit" ; Texture2D::from_file_with_format(exit_bytes, None),
        "crystal_ball" ; Texture2D::from_file_with_format(crystal_ball_bytes, None),
        "meat" ; Texture2D::from_file_with_format(meat_bytes, None),
        "hourglass" ; Texture2D::from_file_with_format(hourglass_bytes, None),
        "x" ; Texture2D::from_file_with_format(x_bytes, None),
        "potion" ; Texture2D::from_file_with_format(potion_bytes, None)
    );

    let scale_params = DrawTextureParams {
        dest_size: Some(vec2(TILE_SIZE, TILE_SIZE)),
        ..Default::default()
    };

    let mut game_state = GameState::Active;
    let mut transition: Option<Transition> = None;

    //player.effects.add(Item::CrystalBall);
    //player.effects.add(Item::Hourglass);
    //player.effects.add(Item::Hourglass);
    //player.effects.add(Item::Hourglass);

    loop {
        clear_background(BLACK);
        if game_state == GameState::Quit {
            break;
        }

        let dt = get_frame_time().min(0.25);
        render_castle(&castle, &player, &zombie, &texture_map, &scale_params);

        if let Some(t) = &mut transition {
            t.remaining -= dt;
            draw_transparant_screen(&t.first_text, &t.second_text, 100.0, 90.0, -50.0, -10.0);
            if t.remaining <= 0.0 {
                transition = None;
            }

            next_frame().await;
            continue;
        }

        player.open_inventory();
        // For effects that arent meant to last for the entire level
        if player.effects.any_active() {
            zombie.freeze(&mut player, &game_state, dt);
            player.replenish_stats();
        }

        if player.in_shop() {
            Controller::shop(&mut player);
        } else if player.in_inventory() {
            //player.inventory.add_item(Item::CrystalBall);
            //player.inventory.add_item(Item::Meat);
            //player.inventory.add_item(Item::Hourglass);
            //player.inventory.add_item(Item::Potion);
            player
                .inventory
                .display(&player, &texture_map, scale_params.clone());
            Controller::inventory(&mut player);
        } else {
            Controller::roam(&castle, &mut player, &mut zombie, &dt, &mut game_state);
        }

        Controller::mutate_game_state(&mut game_state);
        if !matches!(game_state, GameState::Win | GameState::Lose) {
            player.dead();
            player.caught(&zombie);
            game_state = check_game_status(&player, game_state);
        }

        activate_event(
            &mut castle,
            &mut player,
            &mut zombie,
            &texture_map,
            &scale_params,
            &mut game_state,
            &mut transition,
        );

        if matches!(game_state, GameState::Paused) {
            draw_transparant_screen(
                "Game Paused",
                "Press any key to continue.",
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
