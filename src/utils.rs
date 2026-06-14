// TODO: Just keep everything in the prelude for now for shorter import
// statements and incase things need to be added, then clean up later.
pub mod prelude {
    use std::collections::HashMap;

    use macroquad::input::KeyCode;
    use rand::prelude::*;

    use crate::{
        castle::{Castle, Tile},
        events::prelude::EventID,
        player::Player,
    };

    #[derive(PartialEq, Debug)]
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

    pub trait Descent {
        fn increment_floor(&mut self) -> &mut i32;

        fn descend(&mut self) {
            *self.increment_floor() += 1
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

    pub fn reached_final_exit(castle: &Castle, player: &Player) -> bool {
        if player.current_coordinate.z != castle.max_floors() {
            false
        } else {
            // There will always be one exit
            let exit_coordinate = filter_possible_coordinates(
                &castle.layout,
                castle.max_floors(),
                Tile::Door(EventID::Exit),
            )[0];

            (exit_coordinate.x - player.current_coordinate.x) == 0
                && (exit_coordinate.y - player.current_coordinate.y) == 0
        }
    }

    pub fn player_dead(player: &Player) -> bool {
        player.hp <= 0
    }

    pub fn next_level<Entity: Descent>(object: &mut Entity) {
        object.descend();
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
            let mut map = std::collections::HashMap::new();
            $(map.insert($key, $value);)*

            map
        }
    ]
}
