use castledescent::{
    castle::Castle,
    player::Player,
    utils::{Status, check_game_status},
    zombie::Zombie,
};

// Will be used as an initializer of a few things
fn initialize() -> (Castle, Player, Zombie) {
    let mut castle = Castle::generate();
    let mut player = Player::spawn(&castle);
    let mut zombie = Zombie::spawn(&castle, &player);

    (castle, player, zombie)
}

fn main() {
    let (castle, player, zombie) = initialize();
    let pos = &player.current_position;
    println!("{:?}", player.current_position);
    println!("{}", castle.check_object(pos.0, pos.1, pos.2));
    println!("{:?}", zombie.current_position);
    println!("{}", zombie.distance_from_player);

    for i in 0..castle.width {
        for j in 0..castle.depth {
            for k in 0..castle.floors {
                println!("{}", castle.check_object(i, j, k));
            }
        }
    }

    let final_game_status: Status = loop {
        let current_game_state = check_game_status(&castle, &player, &zombie);

        if !matches!(current_game_state, Status::Active) {
            break current_game_state;
        }
    };

    if matches!(final_game_status, Status::Win) {
    } else {
    }
}
