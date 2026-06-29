// PNG assets from https://emoji.aranja.com/
mod castle;
mod controller;
mod events;
mod item;
mod menu;
mod player;
mod utils;
mod zombie;

use macroquad::prelude::*;
use std::collections::HashMap;

use crate::{
    castle::{Castle, Tile},
    controller::Controller,
    events::EventID,
    item::Item,
    menu::{EventMenu, EventMenuAction, ItemMenu, ItemMenuAction, MenuType},
    player::{ActiveMenu, Player, PlayerStatus},
    utils::{Offset, TILE_SIZE, prelude::*},
    zombie::{Zombie, ZombieStatus},
};

enum Flow {
    Continue,
    SkipFrame,
}

struct Transition {
    remaining: f32,
    first_text: String,
    second_text: String,
}

struct Assets {
    textures: HashMap<&'static str, Texture2D>,
    scale: DrawTextureParams,
}

impl Assets {
    fn load() -> Self {
        let textures: HashMap<&str, Texture2D> = hashmap!(
            "door" ; Texture2D::from_file_with_format(include_bytes!("../assets/door.png"), None),
            "merchant" ; Texture2D::from_file_with_format(include_bytes!("../assets/merchant.png"), None),
            "monster" ; Texture2D::from_file_with_format(include_bytes!("../assets/monster.png"), None),
            "fairy" ; Texture2D::from_file_with_format(include_bytes!("../assets/fairy.png"), None),
            "genie" ; Texture2D::from_file_with_format(include_bytes!("../assets/genie.png"), None),
            "player" ; Texture2D::from_file_with_format(include_bytes!("../assets/player.png"), None),
            "zombie" ; Texture2D::from_file_with_format(include_bytes!("../assets/zombie.png"), None),
            "exit" ; Texture2D::from_file_with_format(include_bytes!("../assets/exit.png"), None),
            "crystal_ball" ; Texture2D::from_file_with_format(include_bytes!("../assets/crystal_ball.png"), None),
            "meat" ; Texture2D::from_file_with_format(include_bytes!("../assets/meat.png"), None),
            "hourglass" ; Texture2D::from_file_with_format(include_bytes!("../assets/hourglass.png"), None),
            "x" ; Texture2D::from_file_with_format(include_bytes!("../assets/x.png"), None)
        );

        let scale = DrawTextureParams {
            dest_size: Some(vec2(TILE_SIZE, TILE_SIZE)),
            ..Default::default()
        };

        Self { textures, scale }
    }

    fn tex(&self, name: &str) -> &Texture2D {
        self.textures.get(name).unwrap()
    }

    fn scale_params(&self) -> DrawTextureParams {
        self.scale.clone()
    }
}

struct Game {
    castle: Castle,
    player: Player,
    zombie: Zombie,
    state: GameState,
    transition: Option<Transition>,
    assets: Assets,
}

impl Game {
    fn new() -> Self {
        let castle = Castle::generate();
        let player = Player::spawn(&castle);
        let zombie = Zombie::spawn(&castle, &player);

        Self {
            castle,
            player,
            zombie,
            state: GameState::Start,
            transition: None,
            assets: Assets::load(),
        }
    }

    async fn run(mut self) {
        set_fullscreen(true);

        loop {
            clear_background(BLACK);

            if self.state == GameState::Quit {
                break;
            }

            if self.state == GameState::Start {
                self.start_screen();
                next_frame().await;
                continue;
            }

            let dt = get_frame_time().min(0.25);

            self.render_castle();
            self.render_moving_objects();

            if let Flow::SkipFrame = self.tick_transition(dt) {
                next_frame().await;
                continue;
            }

            self.activate_event();

            if let Flow::SkipFrame = self.update(dt) {
                next_frame().await;
                continue;
            }

            self.resolve_frame();

            if self.state == GameState::Paused {
                draw_transparant_screen(
                    "Game Paused",
                    "Press escape to continue.",
                    0.95,
                    0.91,
                    1.0,
                    1.1,
                );
                self.zombie.update_status(ZombieStatus::Frozen);
            }

            if self.reset_if_over() {
                clear_background(BLACK);
            }

            next_frame().await;
        }
    }

    fn start_screen(&mut self) {
        draw_transparant_screen(
            "Castle Descent",
            "Press 'enter' to start game.",
            0.90,
            0.85,
            0.95,
            1.05,
        );

        Controller::start(&mut self.state);
        if let Some(key) = Controller::get_key() {
            Controller::quit(&key, &mut self.state);
        }
    }

    fn render_castle(&self) {
        let offset = castle_offset(&self.castle);

        if self.player.in_inventory()
            || self.player.in_shop()
            || self.player.encounter.is_playable_event(&self.castle)
        {
            return;
        }

        for (coordinate, tile) in &self.castle.layout {
            if coordinate.z != self.castle.current_floor {
                continue;
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
                        self.assets.tex("merchant"),
                        *coordinate,
                        self.assets.scale_params(),
                        offset,
                    );
                }
                Tile::Door(_) => {
                    let door_type = if self.player.effects.reveal_exit()
                        && matches!(
                            self.castle.get_ref_object(*coordinate),
                            Some(Tile::Door(EventID::Exit))
                        ) {
                        "exit"
                    } else {
                        "door"
                    };

                    draw_asset(
                        self.assets.tex(door_type),
                        *coordinate,
                        self.assets.scale_params(),
                        offset,
                    );
                }
            }
        }

        draw_text(
            format!(
                "Floor {} of {}",
                self.castle.current_floor + 1,
                self.castle.floors
            ),
            screen_width() / 2.0 * 0.92,
            screen_height() * 0.95,
            20.0,
            WHITE,
        );
    }

    fn render_moving_objects(&self) {
        let offset = castle_offset(&self.castle);

        if self.player.in_inventory() || self.player.in_shop() {
            return;
        }

        if self.player.status != PlayerStatus::Hide {
            draw_asset(
                self.assets.tex("player"),
                self.player.current_coordinate,
                self.assets.scale_params(),
                offset,
            );
        }

        if self.player.encounter.is_playable_event(&self.castle) {
            return;
        }

        draw_asset(
            self.assets.tex("zombie"),
            self.zombie.current_coordinate,
            self.assets.scale_params(),
            offset,
        );

        if self.player.effects.freeze_zombie() {
            draw_asset(
                self.assets.tex("x"),
                self.zombie.current_coordinate,
                self.assets.scale_params(),
                offset,
            );
        }
    }

    fn tick_transition(&mut self, dt: f32) -> Flow {
        let Some(t) = &mut self.transition else {
            return Flow::Continue;
        };

        t.remaining -= dt;
        draw_transparant_screen(&t.first_text, &t.second_text, 0.93, 0.945, 0.95, 1.05);
        let finished = t.remaining <= 0.0;

        if finished {
            self.transition = None;
            Flow::Continue
        } else {
            Flow::SkipFrame
        }
    }

    fn activate_event(&mut self) {
        let offset = castle_offset(&self.castle);

        if let Some(tile) = self
            .castle
            .get_mutable_object(self.player.encounter.coordinate)
        {
            match tile {
                Tile::Door(EventID::MonsterEvent(_)) => {
                    if self.player.status == PlayerStatus::Inventory {
                        return;
                    }
                    if self.player.status != PlayerStatus::Event {
                        self.player.update_status(PlayerStatus::Event);
                    }
                    draw_asset(
                        self.assets.tex("monster"),
                        self.player.encounter.coordinate,
                        self.assets.scale_params(),
                        offset,
                    );
                }
                Tile::Door(EventID::FairyEvent(_)) => {
                    if self.player.status == PlayerStatus::Inventory {
                        return;
                    }
                    if self.player.status != PlayerStatus::Event {
                        self.player.update_status(PlayerStatus::Event);
                    }
                    draw_asset(
                        self.assets.tex("fairy"),
                        self.player.encounter.coordinate,
                        self.assets.scale_params(),
                        offset,
                    );
                }
                Tile::Door(EventID::GenieEvent(_)) => {
                    if self.player.status == PlayerStatus::Inventory {
                        return;
                    }
                    if self.player.status != PlayerStatus::Event {
                        self.player.update_status(PlayerStatus::Event);
                    }
                    draw_asset(
                        self.assets.tex("genie"),
                        self.player.encounter.coordinate,
                        self.assets.scale_params(),
                        offset,
                    );
                }
                Tile::Shop => {
                    if self.player.status != PlayerStatus::Shop {
                        self.player.update_status(PlayerStatus::Shop);
                    }
                }
                Tile::Door(EventID::Empty) => {
                    if self.player.status != PlayerStatus::Hide {
                        self.player.update_status(PlayerStatus::Hide);
                    }
                }
                Tile::Door(EventID::Exit) => {
                    if self.player.current_coordinate.z < self.castle.max_floors() {
                        self.castle.increment_floor();
                        self.player.current_coordinate = Player::select_initial_location(
                            &self.castle,
                            self.castle.current_floor,
                        );
                        self.player.encounter.coordinate = self.player.current_coordinate;
                        self.player.effects.inactivate(Item::CrystalBall);

                        self.zombie.current_coordinate = Zombie::select_initial_location(
                            &self.castle,
                            &self.player,
                            self.castle.current_floor,
                        );
                        self.player.update_status(PlayerStatus::Roam);

                        self.transition = Some(Transition {
                            remaining: 2.0,
                            first_text: "Found the exit.".to_string(),
                            second_text: format!("Moving to floor: {}", self.castle.current_floor),
                        });
                    } else {
                        self.player.update_status(PlayerStatus::Win);
                    }
                }
                Tile::Floor => {
                    if !self.player.in_inventory() {
                        self.player.update_status(PlayerStatus::Roam);
                    }
                }
            };
        }
    }

    fn update(&mut self, dt: f32) -> Flow {
        self.player.open_inventory();
        if self.player.effects.any_active() {
            self.zombie.freeze(&mut self.player, &self.state, dt);
            self.player.replenish_stats();
        }

        if (self.player.in_shop() || self.player.in_inventory()) && self.state == GameState::Active
        {
            if self.player.menu.is_none() {
                let kind = if self.player.in_shop() {
                    MenuType::Shop
                } else {
                    MenuType::Inventory
                };
                self.player.menu = Some(ActiveMenu::Item(ItemMenu::open(kind)));
            }

            let items = if self.player.in_shop() {
                Item::item_and_price()
            } else {
                self.player.inventory.storage_to_pairs()
            };
            let max_distance = if self.player.in_shop() {
                Item::max_distance()
            } else {
                self.player.inventory.max_space_distance()
            };

            if let Some(menu) = self.player.menu.take()
                && let ActiveMenu::Item(mut item_menu) = menu
            {
                let kind = item_menu.kind();
                let action = item_menu.display(
                    &self.player,
                    &items,
                    &self.assets.textures,
                    max_distance,
                    self.assets.scale_params(),
                );
                self.player.menu = Some(ActiveMenu::Item(item_menu));

                match action {
                    ItemMenuAction::Close => {
                        self.player.menu = None;
                        self.player.update_status(PlayerStatus::Roam);
                        if *self
                            .castle
                            .get_ref_object(self.player.encounter.coordinate)
                            .unwrap()
                            == Tile::Shop
                        {
                            self.player.encounter.coordinate = self.player.current_coordinate;
                        }
                    }
                    ItemMenuAction::Confirm(item, quantity) => match kind {
                        MenuType::Shop => self.player.buy(item, quantity),
                        MenuType::Inventory => self.player.use_item(item, quantity),
                    },
                    ItemMenuAction::None => {}
                }
            }
        } else if self.player.in_event() && self.state == GameState::Active {
            if self.player.menu.is_none() {
                self.player.menu = Some(ActiveMenu::Event(EventMenu::open()));
            }

            let offset = castle_offset(&self.castle);

            if let Some(menu) = self.player.menu.take()
                && let ActiveMenu::Event(mut event_menu) = menu
            {
                let action = if let Some(Tile::Door(event)) =
                    self.castle.get_ref_object(self.player.encounter.coordinate)
                {
                    event_menu.display(&self.player, event, offset)
                } else {
                    EventMenuAction::None
                };
                self.player.menu = Some(ActiveMenu::Event(event_menu));

                if let Some(Tile::Door(event)) = self
                    .castle
                    .get_mutable_object(self.player.encounter.coordinate)
                {
                    event.resolve(&mut self.player, action);
                }

                let reached_outcome = matches!(
                    self.castle.get_ref_object(self.player.encounter.coordinate),
                    Some(Tile::Door(event)) if event.outcome().is_some()
                );

                if reached_outcome && self.player.hp > 0 {
                    if self.player.event_log.remaining > 0.0 {
                        self.player.event_log.remaining -= dt;
                        return Flow::SkipFrame;
                    }

                    if let Some(Tile::Door(event)) = self
                        .castle
                        .get_mutable_object(self.player.encounter.coordinate)
                    {
                        event.clear_outcome();
                        event.replace_if_complete();
                    }

                    self.player.menu = None;
                    self.player.turn = None;
                    self.player.cooldown = 1.0;
                    self.player.update_status(PlayerStatus::Roam);
                    self.player.encounter.coordinate = self.player.current_coordinate;
                    self.player.event_log.reset();
                }
            }
        } else {
            Controller::roam(
                &self.castle,
                &mut self.player,
                &mut self.zombie,
                &dt,
                &mut self.state,
            );
        }

        Flow::Continue
    }

    fn resolve_frame(&mut self) {
        Controller::mutate_game_state(&mut self.state);

        if !matches!(self.state, GameState::Win | GameState::Lose) {
            self.player.dead();
            self.player.caught(&self.zombie);
            self.state = check_game_status(&self.player, &self.state);
        }
    }

    fn reset_if_over(&mut self) -> bool {
        if reset_game(&mut self.state) {
            self.reinitialize();
            true
        } else {
            false
        }
    }

    fn reinitialize(&mut self) {
        self.castle = Castle::generate();
        self.player = Player::spawn(&self.castle);
        self.zombie = Zombie::spawn(&self.castle, &self.player);
    }
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

fn check_game_status(player: &Player, game_status: &GameState) -> GameState {
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
        );
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
    Game::new().run().await;
}
