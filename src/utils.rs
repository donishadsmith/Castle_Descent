use std::collections::HashMap;

use rand::prelude::*;

use crate::{
    castle::{Castle, Reveal, Tile},
    player::Player,
    zombie::Zombie,
};

pub enum Status {
    Active,
    Lose,
    Win,
}

pub trait Descent {
    fn increment_floor(&mut self) -> &mut i8;

    fn descend(&mut self) {
        *self.increment_floor() += 1
    }
}

pub mod prelude {
    use rand::prelude::*;

    pub fn choose_random_value(mut vec: Vec<i8>) -> i8 {
        let mut rng = rand::rng();
        vec.shuffle(&mut rng);

        vec[0]
    }
}

pub fn choose_random_coordinate(keys: &mut Vec<(i8, i8, i8)>) -> (i8, i8, i8) {
    let mut rng = rand::rng();

    *keys.choose(&mut rng).unwrap()
}

pub fn filter_possible_coordinates(
    layout: &HashMap<(i8, i8, i8), Tile>,
    current_floor: i8,
    filter_type: Tile,
) -> Vec<(i8, i8, i8)> {
    let filtered_keys: Vec<(i8, i8, i8)> = layout
        .iter()
        .filter_map(|(key, tile)| (key.2 == current_floor && tile == &filter_type).then_some(*key))
        .collect();

    filtered_keys
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

pub fn check_game_status(castle: &Castle, player: &Player, zombie: &Zombie) -> Status {
    if player_caught(&player, &zombie) || player_dead(&player) {
        Status::Lose
    } else if reached_final_exit(&castle, &player) {
        Status::Win
    } else {
        Status::Active
    }
}
