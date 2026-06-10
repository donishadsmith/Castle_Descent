use macroquad::prelude::*;

use Castle_Descent::{
    castle::{Castle, Tile},
    player::Player,
    utils::{Status, check_game_status},
    zombie::Zombie,
};

const TILE_SIZE: f32 = 32.0;

// Will be used as an initializer of a few things
fn initialize() -> (Castle, Player, Zombie) {
    let castle = Castle::generate();
    let player = Player::spawn(&castle);
    let zombie = Zombie::spawn(&castle, &player);

    (castle, player, zombie)
}

#[macroquad::main("Castle Descent")]
async fn main() {
    let (mut castle, mut player, mut zombie) = initialize();

    let tex_door = load_texture("assets/door.png").await.unwrap();
    let tex_merchant = load_texture("assets/merchant.png").await.unwrap();
    let tex_monster = load_texture("assets/monster.png").await.unwrap();
    let tex_fairy = load_texture("assets/fairy.png").await.unwrap();
    let tex_genie = load_texture("assets/genie.png").await.unwrap();
    let tex_player = load_texture("assets/player.png").await.unwrap();
    let tex_zombie = load_texture("assets/zombie.png").await.unwrap();

    let scale_params = DrawTextureParams {
        dest_size: Some(vec2(TILE_SIZE, TILE_SIZE)),
        ..Default::default()
    };

    loop {
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
                            &tex_merchant,
                            screen_x,
                            screen_y,
                            WHITE,
                            scale_params.clone(),
                        );
                    }
                    Tile::Door(reveal) => {
                        draw_texture_ex(&tex_door, screen_x, screen_y, WHITE, scale_params.clone());
                    }
                }
            }
        }

        draw_texture_ex(
            &tex_player,
            player.current_position.0 as f32 * TILE_SIZE,
            player.current_position.1 as f32 * TILE_SIZE,
            WHITE,
            scale_params.clone(),
        );
        draw_texture_ex(
            &tex_zombie,
            zombie.current_position.0 as f32 * TILE_SIZE,
            zombie.current_position.1 as f32 * TILE_SIZE,
            WHITE,
            scale_params.clone(),
        );

        next_frame().await
    }
}
