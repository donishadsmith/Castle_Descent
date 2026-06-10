pub trait Descent {
    fn increment_floor(&mut self) -> &mut i8;

    fn descend(&mut self) {
        *self.increment_floor() += 1
    }
}

pub trait Controller {}
