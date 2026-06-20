use crate::{
    math_as,
    menu::EventMenuAction,
    player::{Player, PlayerStatus},
    utils::prelude::*,
};
use macroquad::texture::get_screen_data;
use rand::prelude::*;
use strum::Display;

#[derive(Debug, PartialEq)]
pub struct EventLog {
    pub message: Option<String>,
    pub remaining: f32,
}

impl EventLog {
    pub fn start() -> Self {
        Self {
            message: None,
            remaining: 1.0,
        }
    }

    pub fn encounter_message(&self, identity: &'static str, status: Option<EventStatus>) -> String {
        let message = if status == Some(EventStatus::Uninitiated) {
            format!("You encountered a {}!", identity)
        } else {
            format!("You re-encountered a {}!", identity)
        };

        message
    }

    pub fn update(&mut self, message: String) {
        self.message = Some(message)
    }

    pub fn reset(&mut self) {
        self.message = None;
        self.remaining = 2.0;
    }
}

pub trait PlayableEvent {
    fn options(&self) -> &[&'static str];

    fn count(&self) -> usize {
        self.options().len()
    }
}

fn coin_flip() -> &'static str {
    let mut rng = rand::rng();
    let arr = ["Monster", "Player"];

    arr.choose(&mut rng).unwrap()
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
        if self.status() == Some(EventStatus::Complete) && self.outcome().is_none() {
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

    pub fn identity(&self) -> &'static str {
        match self {
            EventID::MonsterEvent(_) => "Monster",
            EventID::GenieEvent(_) => "Genie",
            EventID::FairyEvent(_) => "Fairy",
            _ => "Other",
        }
    }

    pub fn resolve(&mut self, player: &mut Player, action: EventMenuAction) {
        match self {
            EventID::MonsterEvent(monster) => {
                match monster.turn {
                    Some(_) => {}
                    None => {
                        let who = coin_flip();
                        match who {
                            "Monster" => {
                                monster.turn = Some(true);
                                player.turn = Some(false);
                            }
                            _ => {
                                player.turn = Some(true);
                                monster.turn = Some(false);
                            }
                        }
                    }
                }

                if matches!(monster.status, EventStatus::Uninitiated) {
                    monster.update_status(EventStatus::Initiated);
                }

                monster.penalty(player, action);
            }
            EventID::FairyEvent(fairy) => {
                if matches!(fairy.status, EventStatus::Uninitiated) {
                    fairy.update_status(EventStatus::Initiated);
                }

                fairy.restore_hp(player, action);
            }
            EventID::GenieEvent(genie) => {
                if matches!(genie.status, EventStatus::Uninitiated) {
                    genie.update_status(EventStatus::Initiated);
                }

                genie.increase_stat(player, action);
            }
            _ => {}
        }

        if (matches!(self.status(), Some(EventStatus::Complete))
            || matches!(action, EventMenuAction::Select("Leave")))
            && self.outcome().is_none()
        {
            player.update_status(PlayerStatus::Roam);
            player.encounter.coordinate = player.current_coordinate;
            player.event_log.reset();
        } else {
            player.update_status(PlayerStatus::Event);
        }
    }

    pub fn outcome(&self) -> Option<bool> {
        match self {
            EventID::MonsterEvent(monster) => monster.outcome,
            EventID::FairyEvent(fairy) => fairy.outcome,
            EventID::GenieEvent(genie) => genie.outcome,
            _ => None,
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
    pub options: [&'static str; 2],
    pub outcome: Option<bool>,
}

impl Fairy {
    pub fn spawn() -> Self {
        Self {
            status: EventStatus::Uninitiated,
            options: ["Restore Health", "Leave"],
            outcome: None,
        }
    }

    fn restore_hp(&mut self, player: &mut Player, action: EventMenuAction) {
        if action == EventMenuAction::Select("Restore Health") {
            player.hp = player.hp_limit;
            player.event_log.message = Some(String::from("Full health restored!"));
            self.outcome = Some(true);
            self.update_status(EventStatus::Complete);
        }

        if !self.outcome.is_none() && player.event_log.remaining <= 0.0 {
            self.outcome = None;
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
    pub options: [&'static str; 3],
    pub outcome: Option<bool>,
}

impl Genie {
    pub fn spawn() -> Self {
        Self {
            status: EventStatus::Uninitiated,
            options: ["Increase HP", "Increase Attack", "Leave"],
            outcome: None,
        }
    }

    pub fn increase_stat(&mut self, player: &mut Player, action: EventMenuAction) {
        match action {
            EventMenuAction::Select(_) => {
                self.outcome = Some(true);

                let increase_value = choose_random_range(1..3) * (player.current_coordinate.z + 1);

                if action == EventMenuAction::Select("Increase HP") {
                    let hp_percentage = math_as!(player.hp, player.hp_limit, f32, "div");
                    player.hp_limit += increase_value;
                    player.hp = math_as!(player.hp_limit, hp_percentage, f32, "prod") as i32;

                    player.event_log.message =
                        Some(format!("HP increased by {} points.", increase_value));
                }

                if action == EventMenuAction::Select("Increase Attack") {
                    player.attack_power.0 += increase_value;
                    player.attack_power.1 += increase_value;

                    player.event_log.message = Some(format!(
                        "Attack range increased by {} points.",
                        increase_value
                    ));
                }

                self.update_status(EventStatus::Complete);
            }
            _ => (),
        }

        if !self.outcome.is_none() && player.event_log.remaining <= 0.0 {
            let screen = get_screen_data();
            screen.export_png("genie.png");
            self.outcome = None;
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
    pub options: [&'static str; 2],
    pub turn: Option<bool>,
    pub outcome: Option<bool>,
}

impl Monster {
    pub fn spawn(hp: i32, money: i32, attack_power: (i32, i32)) -> Self {
        Self {
            hp,
            money,
            attack_power,
            status: EventStatus::Uninitiated,
            options: ["Attack", "Leave"],
            turn: None,
            outcome: None,
        }
    }

    fn penalty(&mut self, player: &mut Player, action: EventMenuAction) {
        if action == EventMenuAction::Select("Leave") {
            match self.outcome {
                Some(_) => {
                    if player.event_log.remaining <= 0.0 {
                        self.outcome = None;
                    }
                }
                None => {
                    self.outcome = Some(true);

                    let penalty = self.hp / 4;
                    player.hp -= penalty;
                    player.money -= penalty;
                    self.turn = None;

                    player.event_log.message = Some(format!("-{} to HP and Money.", penalty));
                    //let screen = get_screen_data();
                    //screen.export_png("monster.png");
                }
            }
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
