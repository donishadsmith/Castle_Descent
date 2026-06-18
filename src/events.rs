use macroquad::prelude::*;
use strum::Display;

use crate::{
    controller::Controller,
    menu::EventMenuAction,
    player::{ActiveMenu, Player, PlayerStatus},
    utils::prelude::*,
};

pub trait PlayableEvent {
    fn options(&self) -> &[&'static str];

    fn count(&self) -> usize {
        self.options().len()
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

    pub fn options(&self) -> &[&'static str] {
        match self {
            EventID::MonsterEvent(monster) => monster.options(),
            EventID::FairyEvent(fairy) => fairy.options(),
            EventID::GenieEvent(genie) => genie.options(),
            _ => &[],
        }
    }

    pub fn hp(&self) -> i32 {
        match self {
            EventID::MonsterEvent(monster) => monster.hp,
            _ => 0,
        }
    }

    pub fn count(&self) -> usize {
        match self {
            EventID::MonsterEvent(monster) => monster.count(),
            EventID::GenieEvent(genie) => genie.count(),
            EventID::FairyEvent(fairy) => fairy.count(),
            _ => 0,
        }
    }

    pub fn identity(&self) -> &str {
        match self {
            EventID::MonsterEvent(_) => "monster",
            EventID::GenieEvent(_) => "genie",
            EventID::FairyEvent(_) => "fairy",
            _ => "other",
        }
    }

    pub fn resolve(&mut self, player: &mut Player, action: EventMenuAction) {
        match self {
            EventID::MonsterEvent(monster) => {
                if matches!(monster.status, EventStatus::Uninitiated) {
                    monster.update_status(EventStatus::Initiated);
                }
            }
            EventID::FairyEvent(fairy) => {
                if matches!(fairy.status, EventStatus::Uninitiated) {
                    fairy.update_status(EventStatus::Initiated);
                }
                if matches!(action, EventMenuAction::Select("Heal")) {
                    player.hp = 100;
                    fairy.update_status(EventStatus::Complete);
                }
            }
            EventID::GenieEvent(genie) => {}
            _ => {}
        }

        if matches!(self.status(), Some(EventStatus::Complete))
            || matches!(action, EventMenuAction::Select("Leave"))
        {
            player.update_status(PlayerStatus::Roam);
            player.encounter.coordinate = player.current_coordinate;
        } else {
            player.update_status(PlayerStatus::Event);
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
    pub encounter_text: &'static str,
    pub options: [&'static str; 2],
}

impl Fairy {
    pub fn spawn() -> Self {
        Self {
            status: EventStatus::Uninitiated,
            encounter_text: "You encountered a fairy!",
            options: ["Heal (Full Health)", "Leave"],
        }
    }

    fn restore_hp(player: &mut Player, action: EventMenuAction) {
        if action == EventMenuAction::Select("Heal") {
            player.hp = 100;
        }
    }
}

impl Entity for Fairy {}

impl EntityStatus for Fairy {
    type Status = EventStatus;

    fn current_status(&mut self) -> &mut EventStatus {
        &mut self.status
    }
}

impl PlayableEvent for Fairy {
    fn options(&self) -> &[&'static str] {
        &self.options
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Genie {
    pub status: EventStatus,
    pub encounter_text: &'static str,
    pub options: [&'static str; 3],
}

impl Genie {
    pub fn spawn() -> Self {
        Self {
            status: EventStatus::Uninitiated,
            encounter_text: "You encountered a Genie!",
            options: ["Increase HP", "Increase Attack", "Leave"],
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

impl PlayableEvent for Genie {
    fn options(&self) -> &[&'static str] {
        &self.options
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Monster {
    pub hp: i32,
    pub money: i32,
    pub attack_power: (i32, i32),
    pub status: EventStatus,
    pub encounter_text: &'static str,
    pub options: [&'static str; 2],
}

impl Monster {
    pub fn spawn(hp: i32, money: i32, attack_power: (i32, i32)) -> Self {
        Self {
            hp,
            money,
            attack_power,
            status: EventStatus::Uninitiated,
            encounter_text: "You encountered a Monster!",
            options: ["Attack", "Leave"],
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

impl PlayableEvent for Monster {
    fn options(&self) -> &[&'static str] {
        &self.options
    }
}
