use crate::merchant::Item;
use crate::movement::Descent;
use std::collections::HashMap;

enum PlayerStatus {
    Active,
    Loss,
    Win,
}

pub struct Player {
    hp: i16,
    mana: i16,
    money: i32,
    attack_power: (i16, i16),
    current_position: (i8, i8, i8),
    inventory: HashMap<Item, i16>,
    status: PlayerStatus,
}

/*impl Player {
    pub
}
*/

impl Descent for Player {
    fn increment_floor(&mut self) -> &mut i8 {
        &mut self.current_position.2
    }
}
