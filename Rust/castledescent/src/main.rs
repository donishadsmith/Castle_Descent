use castledescent::castle::Castle;
use castledescent::player::Player;

// Will be used as an initializer of a few things
fn initialize() -> (Castle, Player) {
    let castle = Castle::generate();
    let player = Player::spawn(&castle);

    (castle, player)
}

fn main() {
    let (castle, player) = initialize();
    let pos = &player.current_position;
    println!("{:?}", player.current_position);
    println!("{}", castle.check_object(pos.0, pos.1, pos.2))

    //println!("{:?}", castle.layout);
    /*for i in 0..castle.width {
        for j in 0..castle.depth {
            for k in 0..castle.floors {
                println!("{}", castle.check_object(i, j, k));
            }
        }
    }*/
}
