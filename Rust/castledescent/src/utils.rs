pub mod prelude {
    use rand::prelude::*;
    use std::ops::Range;

    pub fn choose_random_value(range: Range<i8>) -> i8 {
        let mut rng = rand::rng();

        rng.random_range(range)
    }
}
