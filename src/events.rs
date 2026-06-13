pub mod prelude {
    use crate::player::Player;
    use crate::utils::prelude::*;
    use strum::Display;

    #[derive(Clone, Copy, Debug, Display, PartialEq)]
    pub enum EventStatus {
        Incomplete,
        Complete,
    }

    impl StatusType for EventStatus {}

    #[derive(Clone, Copy, Debug, PartialEq)]
    pub struct Fairy {
        status: EventStatus,
    }

    impl Fairy {
        pub fn spawn() -> Self {
            Self {
                status: EventStatus::Incomplete,
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
        status: EventStatus,
    }

    impl Genie {
        pub fn spawn() -> Self {
            Self {
                status: EventStatus::Incomplete,
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
        hp: i8,
        status: EventStatus,
    }

    impl Monster {
        pub fn spawn(hp: i8) -> Self {
            Self {
                hp,
                status: EventStatus::Incomplete,
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
