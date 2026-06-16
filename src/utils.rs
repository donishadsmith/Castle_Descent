pub mod prelude {
    use macroquad::input::KeyCode;
    use rand::prelude::*;
    use std::collections::HashMap;

    use crate::castle::Tile;

    #[derive(PartialEq, Eq, Debug)]
    pub enum GameState {
        Win,
        Lose,
        Paused,
        Active,
        Quit,
    }

    pub enum Component {
        X,
        Y,
        Z,
    }

    #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Ord, PartialOrd)]
    pub struct Coordinate {
        pub x: i32,
        pub y: i32,
        pub z: i32,
    }

    impl Coordinate {
        pub fn new(x: i32, y: i32, z: i32) -> Self {
            Self { x, y, z }
        }

        pub fn to_float(&self, component: Component) -> f32 {
            match component {
                Component::X => self.x as f32,
                Component::Y => self.y as f32,
                Component::Z => self.z as f32,
            }
        }
    }

    pub trait Entity {}
    pub trait StatusType {}
    pub trait EntityStatus: Entity {
        type Status: StatusType;

        fn current_status(&mut self) -> &mut Self::Status;

        fn update_status(&mut self, status: Self::Status) {
            *self.current_status() = status;
        }
    }

    pub fn choose_random_value(values: &[i32]) -> i32 {
        let mut rng = rand::rng();

        *values.choose(&mut rng).unwrap()
    }

    pub fn choose_random_range(range: std::ops::Range<i32>) -> i32 {
        let mut rng = rand::rng();

        rng.random_range(range)
    }

    pub fn choose_random_coordinate(keys: &mut [Coordinate]) -> Coordinate {
        let mut rng = rand::rng();

        *keys.choose(&mut rng).unwrap()
    }

    pub fn filter_possible_coordinates(
        layout: &HashMap<Coordinate, Tile>,
        current_floor: i32,
        filter_type: Tile,
    ) -> Vec<Coordinate> {
        let filtered_keys: Vec<Coordinate> = layout
            .iter()
            .filter_map(|(key, tile)| {
                (key.z == current_floor && tile == &filter_type).then_some(*key)
            })
            .collect();

        filtered_keys
    }

    pub fn get_direction(direction: KeyCode) -> Coordinate {
        match direction {
            KeyCode::Left => Coordinate::new(-1, 0, 0),
            KeyCode::Right => Coordinate::new(1, 0, 0),
            KeyCode::Down => Coordinate::new(0, 1, 0),
            KeyCode::Up => Coordinate::new(0, -1, 0),
            _ => Coordinate::new(0, 0, 0),
        }
    }
}

#[macro_export]
macro_rules! debug_print {
    ($(($statement:expr, $object:expr)),+) => {
        $(println!("{}: {:?}", $statement, $object);)*
    }
}

#[macro_export]
macro_rules! hashmap {
    ($($key:expr ; $value:expr),+) => [
        {
            let mut map = HashMap::new();
            $(map.insert($key, $value);)*

            map
        }
    ]
}
