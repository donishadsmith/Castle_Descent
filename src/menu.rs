use std::collections::HashMap;

use ::macroquad::prelude::*;

use crate::utils::TILE_SIZE;
use crate::{controller::Controller, item::Item, player::Player};

#[derive(Clone, Copy, PartialEq)]
pub enum MenuType {
    Inventory,
    Shop,
}

pub enum MenuAction {
    None,
    Confirm(Item, i32),
    Close,
}

#[derive(Clone, Copy, PartialEq)]
enum MenuMode {
    Browse,
    Quantity { amount: i32 },
}

struct Cursor {
    index: isize,
}

impl Cursor {
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

    fn selected(&self, items: &[(Item, i32)]) -> Option<Item> {
        items.get(self.current(items.len())).map(|(item, _)| *item)
    }
}

pub struct Menu {
    kind: MenuType,
    cursor: Cursor,
    mode: MenuMode,
}

impl Menu {
    pub fn open(kind: MenuType) -> Self {
        Self {
            kind,
            cursor: Cursor::start(),
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
    ) -> MenuAction {
        let key = Controller::get_press();

        let mut action = MenuAction::None;
        self.mode = match (self.mode, key) {
            (MenuMode::Browse, Some(KeyCode::Enter)) if !items.is_empty() => {
                MenuMode::Quantity { amount: 0 }
            }
            (MenuMode::Browse, Some(KeyCode::Backspace)) => {
                action = MenuAction::Close;
                MenuMode::Browse
            }
            (MenuMode::Quantity { .. }, Some(KeyCode::Backspace)) => MenuMode::Browse,
            (MenuMode::Quantity { amount }, Some(KeyCode::Enter)) => {
                if let Some(item) = self.cursor.selected(items) {
                    action = MenuAction::Confirm(item, amount);
                }

                MenuMode::Browse
            }
            // Allow Quantity persist across frames
            (other, _) => other,
        };

        match &mut self.mode {
            MenuMode::Browse => self.cursor.scroll(key, items.len(), false),
            MenuMode::Quantity { amount } => match key {
                Some(KeyCode::Left) => {
                    let item = self.cursor.selected(items).unwrap();
                    *amount -= 1;
                    *amount = amount.rem_euclid(player.item_limit(item, &self.kind) + 1);
                }
                Some(KeyCode::Right) => {
                    let item = self.cursor.selected(items).unwrap();
                    *amount += 1;
                    *amount = amount.rem_euclid(player.item_limit(item, &self.kind) + 1);
                }
                _ => {}
            },
        }

        self.draw(player, items, texture_map, max_distance, scale_params);

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
        let selected_row = self.cursor.current(items.len());

        let mut y = screen_y + max_distance;

        for (index, (item, n)) in items.iter().enumerate() {
            draw_texture_ex(
                texture_map.get(item.identity_to_str()).unwrap(),
                screen_x * 0.40,
                y,
                WHITE,
                scale_params.clone(),
            );

            let y_anchor = y + TILE_SIZE / 2.0;
            draw_text(
                n.to_string(),
                screen_x * 0.42 + TILE_SIZE + 8.0,
                y_anchor + 3.0,
                24.0,
                WHITE,
            );

            if index == selected_row {
                // y is top of asset
                draw_triangle(
                    vec2(screen_x * 0.38 - 8.0, y_anchor),
                    vec2(screen_x * 0.38 - 24.0, y_anchor - 10.0),
                    vec2(screen_x * 0.38 - 24.0, y_anchor + 10.0),
                    YELLOW,
                );

                let description_start_x = screen_x * 0.50 + TILE_SIZE + 8.0;
                if let Some(item) = self.cursor.selected(items) {
                    draw_text(
                        item.description(),
                        description_start_x,
                        y_anchor,
                        20.0,
                        WHITE,
                    );

                    let dims = measure_text(&item.description(), None, 20, 1.0);
                    if let MenuMode::Quantity { amount } = self.mode {
                        self.draw_quantity(
                            description_start_x + dims.width + TILE_SIZE * 2.0,
                            y_anchor - 4.0,
                            amount,
                        );
                    }
                }
            }

            y += 60.0;
        }

        let (stats, help) = self.text(player);
        draw_text(&stats, screen_x * 0.8, screen_y * 1.3, 30.0, WHITE);

        let shift_text_x = if matches!(self.kind, MenuType::Shop) {
            0.58
        } else {
            0.77
        };
        draw_text(&help, screen_x * shift_text_x, screen_y * 1.4, 20.0, WHITE);
    }

    fn draw_quantity(&self, center_x: f32, y_anchor: f32, amount: i32) {
        let gap = 30.0;
        let len = 16.0;
        let half_h = 10.0;

        // Left triangle
        draw_triangle(
            vec2(center_x - gap - len, y_anchor),
            vec2(center_x - gap, y_anchor - half_h),
            vec2(center_x - gap, y_anchor + half_h),
            YELLOW,
        );
        // Right triangle
        draw_triangle(
            vec2(center_x + gap + len, y_anchor),
            vec2(center_x + gap, y_anchor - half_h),
            vec2(center_x + gap, y_anchor + half_h),
            YELLOW,
        );

        let text = amount.to_string();
        let dims = measure_text(&text, None, 30, 1.0);
        draw_text(
            &text,
            center_x - dims.width / 2.0,
            y_anchor + dims.height / 2.0,
            30.0,
            WHITE,
        );
    }

    fn text(&self, player: &Player) -> (String, String) {
        let stats = match self.kind {
            MenuType::Inventory => format!(
                "HP: {} | Mana: {} | Money: {}",
                player.hp, player.mana, player.money
            ),
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
