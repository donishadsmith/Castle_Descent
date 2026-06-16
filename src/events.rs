use macroquad::input::KeyCode;
use strum::Display;

use crate::{
    controller::Controller,
    player::{Player, PlayerStatus},
    utils::prelude::*,
};

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
    pub fn activate(&mut self, player: &mut Player, game_state: &mut GameState) {
        if *game_state != GameState::Active {
            return;
        }

        let Some(key) = Controller::get_key() else {
            return;
        };

        match self {
            EventID::MonsterEvent(monster) => {
                if matches!(monster.status, EventStatus::Uninitiated) {
                    monster.update_status(EventStatus::Initiated);
                }

                //monster.update_status(EventStatus::Complete);
                Self::escape_event(player, &key, monster);
            }
            EventID::FairyEvent(fairy) => {
                if matches!(fairy.status, EventStatus::Uninitiated) {
                    fairy.update_status(EventStatus::Initiated);
                }

                Self::escape_event(player, &key, fairy)
            }
            EventID::GenieEvent(genie) => {
                if matches!(genie.status, EventStatus::Uninitiated) {
                    genie.update_status(EventStatus::Initiated);
                }

                Self::escape_event(player, &key, genie)
            }
            _ => (),
        }
    }

    fn escape_event<T: EntityStatus<Status = EventStatus>>(
        player: &mut Player,
        key: &KeyCode,
        entity: &mut T,
    ) {
        // Eventually will replace with logic for running away, E is just
        // for testing and escaping for now
        if matches!(key, KeyCode::E) || matches!(entity.current_status(), EventStatus::Complete) {
            player.update_status(PlayerStatus::Roam);
            player.encounter.coordinate = player.current_coordinate;
        } else {
            player.update_status(PlayerStatus::Event);
        }
    }

    pub fn status(&self) -> Option<EventStatus> {
        match self {
            EventID::MonsterEvent(monster) => Some(monster.status),
            EventID::FairyEvent(fairy) => Some(fairy.status),
            EventID::GenieEvent(genie) => Some(genie.status),
            _ => None,
        }
    }

    pub fn replace_if_complete(&mut self) {
        if self.status() == Some(EventStatus::Complete) {
            *self = EventID::Empty;
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
    pub hp: i32,
    pub status: EventStatus,
}

impl Monster {
    pub fn spawn(hp: i32) -> Self {
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
