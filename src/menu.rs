use std::collections::HashMap;

use macroquad::prelude::*;
use strum::Display;

use crate::{
    controller::Controller,
    events::EventID,
    item::Item,
    player::Player,
    utils::{Offset, TILE_SIZE},
};

#[derive(Clone, Copy, PartialEq)]
pub enum MenuType {
    Inventory,
    Shop,
}

pub enum ItemMenuAction {
    None,
    Confirm(Item, i32),
    Close,
}

#[derive(Clone, Copy, PartialEq)]
enum MenuMode {
    Browse,
    Quantity { amount: i32 },
}

pub struct MenuCursor {
    index: isize,
}

impl MenuCursor {
    fn start() -> Self {
        Self { index: 0 }
    }

    fn scroll(&mut self, key: Option<KeyCode>, len: usize, is_event: bool) {
        if !is_event {
            if len == 0 {
                return;
            }
            match key {
                Some(KeyCode::Up) => self.index -= 1,
                Some(KeyCode::Down) => self.index += 1,
                _ => {}
            }
        } else {
            match key {
                Some(KeyCode::Left) => self.index -= 1,
                Some(KeyCode::Right) => self.index += 1,
                _ => {}
            }
        }

        self.index = self.index.rem_euclid(len as isize);
    }

    fn current(&self, len: usize) -> usize {
        if len == 0 {
            0
        } else {
            self.index.rem_euclid(len as isize) as usize
        }
    }

    fn item_selected(&self, items: &[(Item, i32)]) -> Option<Item> {
        items.get(self.current(items.len())).map(|(item, _)| *item)
    }

    fn text_selected(&self, options: &[&'static str]) -> Option<&'static str> {
        options.get(self.current(options.len())).copied()
    }
}

pub struct ItemMenu {
    kind: MenuType,
    cursor: MenuCursor,
    mode: MenuMode,
}

impl ItemMenu {
    pub fn open(kind: MenuType) -> Self {
        Self {
            kind,
            cursor: MenuCursor::start(),
            mode: MenuMode::Browse,
        }
    }

    pub fn kind(&self) -> MenuType {
        self.kind
    }

    pub fn display(
        &mut self,
        player: &Player,
        items: &[(Item, i32)],
        texture_map: &HashMap<&str, Texture2D>,
        max_distance: f32,
        scale_params: DrawTextureParams,
    ) -> ItemMenuAction {
        let key = Controller::get_press();
        let mut action = ItemMenuAction::None;

        self.draw(player, items, texture_map, max_distance, scale_params);

        self.mode = match (self.mode, key) {
            (MenuMode::Browse, Some(KeyCode::Enter)) => {
                if player.in_inventory() && player.inventory.is_empty() {
                    MenuMode::Browse
                } else {
                    MenuMode::Quantity { amount: 0 }
                }
            }
            (MenuMode::Browse, Some(KeyCode::Backspace)) => {
                action = ItemMenuAction::Close;
                MenuMode::Browse
            }
            (MenuMode::Quantity { .. }, Some(KeyCode::Backspace)) => MenuMode::Browse,
            (MenuMode::Quantity { amount }, Some(KeyCode::Enter)) => {
                if let Some(item) = self.cursor.item_selected(items) {
                    action = ItemMenuAction::Confirm(item, amount);
                }

                MenuMode::Browse
            }
            // Allow Quantity persist across frames
            (other, _) => other,
        };

        match &mut self.mode {
            MenuMode::Browse => self.cursor.scroll(key, items.len(), false),
            MenuMode::Quantity { amount } => {
                if key == Some(KeyCode::Left) {
                    *amount -= 1
                } else if key == Some(KeyCode::Right) {
                    *amount += 1
                }

                let item = self.cursor.item_selected(items).unwrap();
                *amount = amount.rem_euclid(player.item_limit(item, &self.kind) + 1)
            }
        }

        /*
        if self.kind == MenuType::Inventory {
            let screen = get_screen_data();
            screen.export_png("inventory.png");
        }
        if self.kind == MenuType::Shop {
            let screen = get_screen_data();
            screen.export_png("merchant.png");
        }*/

        action
    }

    fn draw(
        &self,
        player: &Player,
        items: &[(Item, i32)],
        texture_map: &HashMap<&str, Texture2D>,
        max_distance: f32,
        scale_params: DrawTextureParams,
    ) {
        let screen_x = screen_width() / 2.0;
        let screen_y = screen_height() / 2.0;
        let item_selected_row = self.cursor.current(items.len());

        let mut shift_y = screen_y + max_distance;

        for (index, (item, n)) in items.iter().enumerate() {
            draw_texture_ex(
                texture_map.get(item.identity_to_str()).unwrap(),
                screen_x * 0.40,
                shift_y,
                WHITE,
                scale_params.clone(),
            );

            let anchor_y = shift_y + TILE_SIZE / 2.0;
            draw_text(
                n.to_string(),
                screen_x * 0.42 + TILE_SIZE + 8.0,
                anchor_y + 3.0,
                24.0,
                WHITE,
            );

            if index == item_selected_row {
                // y is top of asset
                draw_triangle(
                    vec2(screen_x * 0.38 - 8.0, anchor_y),
                    vec2(screen_x * 0.38 - 24.0, anchor_y - 10.0),
                    vec2(screen_x * 0.38 - 24.0, anchor_y + 10.0),
                    YELLOW,
                );

                let description_start_x = screen_x * 0.50 + TILE_SIZE + 8.0;
                if let Some(item) = self.cursor.item_selected(items) {
                    draw_text(
                        item.description(),
                        description_start_x,
                        anchor_y,
                        20.0,
                        WHITE,
                    );

                    let dims = measure_text(item.description(), None, 20, 1.0);
                    if let MenuMode::Quantity { amount } = self.mode {
                        self.draw_quantity(
                            description_start_x + dims.width + TILE_SIZE * 2.0,
                            anchor_y - 4.0,
                            amount,
                        );
                    }
                }
            }

            shift_y += 60.0;
        }

        let (stats, help) = self.text(player);
        draw_text(&stats, screen_x * 0.8, screen_y * 1.1, 30.0, WHITE);

        let shift_text_x = if matches!(self.kind, MenuType::Shop) {
            0.58
        } else {
            0.64
        };
        draw_text(&help, screen_x * shift_text_x, screen_y * 1.2, 20.0, WHITE);
    }

    fn draw_quantity(&self, center_x: f32, anchor_y: f32, amount: i32) {
        let gap = 30.0;
        let len = 16.0;
        let half_h = 10.0;

        // Left triangle
        draw_triangle(
            vec2(center_x - gap - len, anchor_y),
            vec2(center_x - gap, anchor_y - half_h),
            vec2(center_x - gap, anchor_y + half_h),
            YELLOW,
        );
        // Right triangle
        draw_triangle(
            vec2(center_x + gap + len, anchor_y),
            vec2(center_x + gap, anchor_y - half_h),
            vec2(center_x + gap, anchor_y + half_h),
            YELLOW,
        );

        let text = amount.to_string();
        let dims = measure_text(&text, None, 30, 1.0);
        draw_text(
            &text,
            center_x - dims.width / 2.0,
            anchor_y + dims.height / 2.0,
            30.0,
            WHITE,
        );
    }

    fn text(&self, player: &Player) -> (String, String) {
        let stats = match self.kind {
            MenuType::Inventory => format!("HP: {} | Money: {}", player.hp, player.money),
            MenuType::Shop => format!("Money: {}", player.money),
        };

        let help = match self.mode {
            MenuMode::Browse => {
                "Navigate (Up/Down) | Select (Enter) | Exit (Backspace)".to_string()
            }
            MenuMode::Quantity { .. } => {
                "Quantity (Left/Right) | Confirm (Enter) | Back (Backspace)".to_string()
            }
        };

        (stats, help)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Display)]
pub enum EventMenuAction {
    None,
    Browse,
    Select(&'static str),
}

pub struct EventMenu {
    pub mode: EventMenuAction,
    cursor: MenuCursor,
}

impl EventMenu {
    pub fn open() -> Self {
        Self {
            mode: EventMenuAction::Browse,
            cursor: MenuCursor::start(),
        }
    }

    pub fn display(
        &mut self,
        player: &Player,
        entity: &EventID,
        offset: Offset,
    ) -> EventMenuAction {
        let key = Controller::get_press();
        let mut action = EventMenuAction::None;

        match key {
            Some(KeyCode::Enter) => {
                if let Some(text) = self.cursor.text_selected(entity.options()) {
                    action = EventMenuAction::Select(text);
                }
            }
            _ => {
                if entity.outcome().is_none() {
                    self.cursor.scroll(key, entity.count(), true);
                }
            }
        }

        self.draw(player, entity, offset);

        action
    }

    fn draw(&self, player: &Player, entity: &EventID, mut offset: Offset) {
        let screen_y = screen_height();

        offset.x *= 1.25;

        let log_message = match &player.event_log.message {
            Some(message) => message,
            None => &player.event_log.encounter_message(entity.identity()),
        };

        draw_text(log_message, offset.x + 30.0, screen_y * 0.86, 20.0, WHITE);

        draw_text(
            self.stats_text(player, entity),
            offset.x + 30.0,
            screen_y * 0.88,
            20.0,
            WHITE,
        );

        let width = match entity.identity() {
            "Genie" => 500.0,
            "Fairy" => 400.0,
            _ => 300.0,
        };

        let adjust_x = match entity.identity() {
            "Genie" => -60.0,
            "Fairy" => -40.0,
            _ => 20.0,
        };

        draw_rectangle_lines(
            offset.x + adjust_x,
            screen_height() * 0.90,
            width,
            70.0,
            2.0,
            WHITE,
        );

        let padding = 30.0;
        let mut text_shift_x = 0.0;

        for (index, option) in entity.options().iter().enumerate() {
            let anchor_x = offset.x + adjust_x + 70.0 + text_shift_x;

            draw_text(option, anchor_x, screen_y * 0.94, 20.0, WHITE);

            if index == self.cursor.current(entity.options().len()) {
                let y = screen_y * 0.94 - 6.0;
                draw_triangle(
                    vec2(anchor_x - 8.0, y),
                    vec2(anchor_x - 24.0, y - 8.0),
                    vec2(anchor_x - 24.0, y + 8.0),
                    YELLOW,
                );
            }

            let dims = measure_text(option, None, 20, 1.0);
            text_shift_x += dims.width + padding;
        }
    }

    fn stats_text(&self, player: &Player, entity: &EventID) -> String {
        match entity.identity() {
            "Monster" => format!("Monster HP: {} | Player HP: {}", entity.hp(), player.hp),
            "Genie" => format!(
                "Player HP: {} | Attack Range: Min={}, Max={}",
                player.hp, player.attack_power.0, player.attack_power.1
            ),
            _ => format!("Player HP: {}", player.hp),
        }
    }
}
