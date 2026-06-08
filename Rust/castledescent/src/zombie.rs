use crate::movement::Descent;

pub struct Zombie {
    pub halt: bool,
    pub current_position: (i8, i8, i8),
    pub distance_from_player: u32,
}

impl Descent for Zombie {
    fn increment_floor(&mut self) -> &mut i8 {
        &mut self.current_position.2
    }
}
