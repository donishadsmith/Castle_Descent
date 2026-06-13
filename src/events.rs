pub mod prelude {
    use macroquad::input::{KeyCode, get_keys_down, get_keys_pressed};
    use strum::Display;

    use crate::player::{Player, PlayerStatus};
    use crate::utils::prelude::*;
    use crate::zombie::Zombie;

    fn escape_event(player: &mut Player) {
        // Eventually will replace with logic for running away
        if matches!(get_keys_pressed().iter().next().cloned(), Some(KeyCode::T))
            || matches!(get_keys_down().iter().next().cloned(), Some(KeyCode::T))
        {
            player.update_status(PlayerStatus::Roam);
            player.intended_coordinate = player.current_coordinate;
        } else {
            player.update_status(PlayerStatus::Event);
        }
    }

    #[derive(Clone, Copy, Debug, Display, PartialEq)]
    pub enum EventID {
        MonsterEvent(Monster),
        GenieEvent(Genie),
        FairyEvent(Fairy),
        Empty,
        Exit,
    }

    // Just keep sequences in one implementation
    impl EventID {
        pub fn activate(
            &mut self,
            player: &mut Player,
            zombie: &mut Zombie,
            game_state: &mut GameState,
        ) {
            match self {
                EventID::MonsterEvent(monster @ _) => {
                    if matches!(monster.status, EventStatus::Uninitiated) {
                        monster.update_status(EventStatus::Initiated);
                    }

                    if matches!(monster.status, EventStatus::Complete) {
                        // function that replaces with empty door event
                    }

                    escape_event(player)
                }
                EventID::FairyEvent(fairy @ _) => {
                    if matches!(fairy.status, EventStatus::Uninitiated) {
                        fairy.update_status(EventStatus::Initiated);
                    }

                    if matches!(fairy.status, EventStatus::Complete) {
                        // function that replaces with empty door event
                    }

                    escape_event(player)
                }
                EventID::GenieEvent(genie @ _) => {
                    if matches!(genie.status, EventStatus::Uninitiated) {
                        genie.update_status(EventStatus::Initiated);
                    }

                    if matches!(genie.status, EventStatus::Complete) {
                        // function that replaces with empty door event
                    }

                    escape_event(player)
                }
                EventID::Empty => {}
                EventID::Exit => {}
            }
        }
    }

    #[derive(Clone, Copy, Debug, Display, PartialEq)]
    pub enum EventStatus {
        Uninitiated,
        Initiated,
        Complete,
    }

    impl StatusType for EventStatus {}

    #[derive(Clone, Copy, Debug, PartialEq)]
    pub struct Fairy {
        pub status: EventStatus,
    }

    impl Fairy {
        pub fn spawn() -> Self {
            Self {
                status: EventStatus::Uninitiated,
            }
        }

        pub fn restore_hp(player: &mut Player) {
            player.hp = 100;
        }
    }

    impl Entity for Fairy {}

    impl EntityStatus for Fairy {
        type Status = EventStatus;

        fn current_status(&mut self) -> &mut EventStatus {
            &mut self.status
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq)]
    pub struct Genie {
        pub status: EventStatus,
    }

    impl Genie {
        pub fn spawn() -> Self {
            Self {
                status: EventStatus::Uninitiated,
            }
        }
    }

    impl Entity for Genie {}

    impl EntityStatus for Genie {
        type Status = EventStatus;

        fn current_status(&mut self) -> &mut EventStatus {
            &mut self.status
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq)]
    pub struct Monster {
        pub hp: i8,
        pub status: EventStatus,
    }

    impl Monster {
        pub fn spawn(hp: i8) -> Self {
            Self {
                hp,
                status: EventStatus::Uninitiated,
            }
        }
    }

    impl Entity for Monster {}

    impl EntityStatus for Monster {
        type Status = EventStatus;

        fn current_status(&mut self) -> &mut EventStatus {
            &mut self.status
        }
    }
}
