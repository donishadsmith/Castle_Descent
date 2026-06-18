/*
TODO:
- Add logic for Playable events (Monster, Fairy, Genie), includes the menu for each (final major feature).
- Npte, leaving monster without definiting drains hp and money.
*/

// PNG assets from https://emoji.aranja.com/

use ::std::collections::HashMap;
use macroquad::prelude::*;

use castle_descent::{
    castle::{Castle, Tile},
    controller::Controller,
    debug_print,
    events::EventID,
    hashmap,
    item::Item,
    menu::{EventMenu, EventMenuAction, ItemMenu, ItemMenuAction, MenuType},
    player::{ActiveMenu, Player, PlayerStatus},
    utils::{Offset, TILE_SIZE, prelude::*},
    zombie::{Zombie, ZombieStatus},
};

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

pub fn castle_offset(castle: &Castle) -> Offset {
    let offset_x = (screen_width() - castle.width as f32 * TILE_SIZE) / 2.0;
    let offset_y = (screen_height() - castle.depth as f32 * TILE_SIZE) / 2.0;

    Offset::new(offset_x, offset_y)
}

fn draw_asset(
    asset: &Texture2D,
    coordinate: Coordinate,
    scale_params: DrawTextureParams,
    offset: Offset,
) {
    draw_texture_ex(
        asset,
        offset.x + coordinate.to_float(Component::X) * TILE_SIZE,
        offset.y + coordinate.to_float(Component::Y) * TILE_SIZE,
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
    let offset = castle_offset(castle);

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
                offset,
            );
        }

        // Only render player an object on screen if in playable event
        if player.encounter.is_playable_event(castle) {
            return;
        }

        match tile {
            Tile::Floor => {
                draw_rectangle(
                    offset.x + coordinate.to_float(Component::X) * TILE_SIZE,
                    offset.y + coordinate.to_float(Component::Y) * TILE_SIZE,
                    TILE_SIZE,
                    TILE_SIZE,
                    BLACK,
                );
            }
            Tile::Shop => {
                draw_asset(
                    texture_map.get("merchant").unwrap(),
                    *coordinate,
                    scale_params.clone(),
                    offset,
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
                    offset,
                );
            }
        }
    }

    draw_asset(
        texture_map.get("zombie").unwrap(),
        zombie.current_coordinate,
        scale_params.clone(),
        offset,
    );

    if player.effects.freeze_zombie() {
        draw_asset(
            texture_map.get("x").unwrap(),
            zombie.current_coordinate,
            scale_params.clone(),
            offset,
        );
    }

    draw_text(
        format!("Floor {} of {}", castle.current_floor + 1, castle.floors),
        screen_width() / 2.0 * 0.92,
        screen_height() * 0.90,
        20.0,
        WHITE,
    );
}

fn activate_event(
    castle: &mut Castle,
    player: &mut Player,
    zombie: &mut Zombie,
    texture_map: &HashMap<&str, Texture2D>,
    scale_params: &DrawTextureParams,
    transition: &mut Option<Transition>,
) {
    let offset = castle_offset(castle);
    if let Some(tile) = castle.get_mutable_object(player.encounter.coordinate) {
        match tile {
            Tile::Door(EventID::MonsterEvent(_)) => {
                if player.status == PlayerStatus::Inventory {
                    return;
                }

                if player.status != PlayerStatus::Event {
                    player.update_status(PlayerStatus::Event);
                }

                draw_asset(
                    texture_map.get("monster").unwrap(),
                    player.encounter.coordinate,
                    scale_params.clone(),
                    offset,
                );
            }
            Tile::Door(EventID::FairyEvent(_)) => {
                if player.status == PlayerStatus::Inventory {
                    return;
                }

                if player.status != PlayerStatus::Event {
                    player.update_status(PlayerStatus::Event);
                }

                draw_asset(
                    texture_map.get("fairy").unwrap(),
                    player.encounter.coordinate,
                    scale_params.clone(),
                    offset,
                );
            }
            Tile::Door(EventID::GenieEvent(_)) => {
                if player.status == PlayerStatus::Inventory {
                    return;
                }

                if player.status != PlayerStatus::Event {
                    player.update_status(PlayerStatus::Event);
                }

                draw_asset(
                    texture_map.get("genie").unwrap(),
                    player.encounter.coordinate,
                    scale_params.clone(),
                    offset,
                );
            }
            Tile::Shop => {
                if player.status != PlayerStatus::Shop {
                    player.update_status(PlayerStatus::Shop);
                }
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
        screen_width() / 2.0 * first_x_shift,
        screen_height() / 2.0 * first_y_shift,
        30.0,
        WHITE,
    );
    draw_text(
        second_text,
        screen_width() / 2.0 * second_x_shift,
        screen_height() / 2.0 * second_y_shift,
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
            0.97,
            0.85,
            0.95,
            1.05,
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
    set_fullscreen(true);

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
        "x" ; Texture2D::from_file_with_format(x_bytes, None)
    );

    let scale_params = DrawTextureParams {
        dest_size: Some(vec2(TILE_SIZE, TILE_SIZE)),
        ..Default::default()
    };

    let mut game_state = GameState::Active;
    let mut transition: Option<Transition> = None;

    loop {
        clear_background(BLACK);

        if game_state == GameState::Quit {
            break;
        }

        let dt = get_frame_time().min(0.25);
        render_castle(&castle, &player, &zombie, &texture_map, &scale_params);

        if let Some(t) = &mut transition {
            t.remaining -= dt;
            draw_transparant_screen(&t.first_text, &t.second_text, 0.93, 0.945, 0.95, 1.05);
            if t.remaining <= 0.0 {
                transition = None;
            }

            next_frame().await;
            continue;
        }

        player.open_inventory();
        if player.effects.any_active() {
            zombie.freeze(&mut player, &game_state, dt);
            player.replenish_stats();
        }

        if (player.in_shop() || player.in_inventory()) && game_state == GameState::Active {
            if player.menu.is_none() {
                let kind = if player.in_shop() {
                    MenuType::Shop
                } else {
                    MenuType::Inventory
                };

                player.menu = Some(ActiveMenu::Item(ItemMenu::open(kind)));
            }

            let items = if player.in_shop() {
                Item::item_and_price()
            } else {
                player.inventory.storage_to_pairs()
            };
            let max_distance = if player.in_shop() {
                Item::max_distance()
            } else {
                player.inventory.max_space_distance()
            };

            if let Some(menu) = player.menu.take() {
                match menu {
                    ActiveMenu::Item(mut item_menu) => {
                        let kind = item_menu.kind();
                        let action = item_menu.display(
                            &player,
                            &items,
                            &texture_map,
                            max_distance,
                            scale_params.clone(),
                        );

                        player.menu = Some(ActiveMenu::Item(item_menu));

                        match action {
                            ItemMenuAction::Close => {
                                player.menu = None;
                                player.update_status(PlayerStatus::Roam);
                                if *castle.get_ref_object(player.encounter.coordinate).unwrap()
                                    == Tile::Shop
                                {
                                    player.encounter.coordinate = player.current_coordinate;
                                }
                            }
                            ItemMenuAction::Confirm(item, quantity) => match kind {
                                MenuType::Shop => player.buy(item, quantity),
                                MenuType::Inventory => player.use_item(item, quantity),
                            },
                            ItemMenuAction::None => {}
                        }
                    }

                    _ => (),
                }
            }
        } else if player.in_event() && game_state == GameState::Active {
            if player.menu.is_none() {
                player.menu = Some(ActiveMenu::Event(EventMenu::open()));
            }

            let offset = castle_offset(&castle);

            if let Some(menu) = player.menu.take() {
                match menu {
                    ActiveMenu::Event(mut event_menu) => {
                        let action = if let Some(Tile::Door(event)) =
                            castle.get_ref_object(player.encounter.coordinate)
                        {
                            event_menu.display(&player, event, offset)
                        } else {
                            EventMenuAction::None
                        };
                        player.menu = Some(ActiveMenu::Event(event_menu));

                        if let Some(Tile::Door(event)) =
                            castle.get_mutable_object(player.encounter.coordinate)
                        {
                            event.resolve(&mut player, action);
                        }
                        if matches!(action, EventMenuAction::Select("Leave")) {
                            player.menu = None;
                        }
                    }
                    _ => (),
                }
            }
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
            &mut transition,
        );

        if matches!(game_state, GameState::Paused) {
            draw_transparant_screen(
                "Game Paused",
                "Press escape to continue.",
                0.95,
                0.91,
                1.0,
                1.1,
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
