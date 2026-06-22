use crate::{
    menu::EventMenuAction,
    player::{Player, PlayerStatus},
    utils::{Attack, AttackType, prelude::*},
};

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

    pub fn encounter_message(&self, identity: &'static str) -> String {
        format!("You encountered a {}!", identity)
    }

    pub fn update(&mut self, message: String) {
        self.message = Some(message)
    }

    pub fn reset(&mut self) {
        self.message = None;
        self.remaining = 1.0;
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

fn combat(monster: &mut Monster, player: &mut Player) {
    if player.turn.is_none() || monster.outcome.is_some() {
        return;
    }

    let mut lines: Vec<String> = Vec::new();

    if player.turn == Some(true) {
        lines.push(player_attacks(monster, player));

        if monster.outcome.is_none() {
            lines.push(monster_attacks(monster, player));
        }
    } else {
        lines.push(monster_attacks(monster, player));

        if player.hp > 0 {
            lines.push(player_attacks(monster, player));
        }
    }

    player.event_log.message = Some(lines.join(" | "));
}

fn player_attacks(monster: &mut Monster, player: &mut Player) -> String {
    let (attack_type, damage) = player.damage_roll();
    monster.hp -= damage;
    monster.clip_hp();

    if monster.hp <= 0 {
        player.money += monster.money;
        monster.outcome = Some(true);
        monster.update_status(EventStatus::Complete);

        return format!("Monster defeated! +{} money", monster.money);
    }

    combat_text("Player", attack_type, damage)
}

fn monster_attacks(monster: &Monster, player: &mut Player) -> String {
    let (attack_type, damage) = monster.damage_roll();
    player.hp -= damage;
    player.clip_stats();

    combat_text("Monster", attack_type, damage)
}

fn combat_text(identity: &'static str, attack_type: AttackType, damage: i32) -> String {
    let critical = if attack_type == AttackType::Critical {
        "Critical! "
    } else {
        ""
    };

    format!("{critical}{identity} dealt {damage} damage")
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
                if matches!(monster.status, EventStatus::Uninitiated) {
                    monster.update_status(EventStatus::Initiated);
                }

                if player.turn.is_none()
                    && monster.outcome.is_none()
                    && action == EventMenuAction::Select("Attack")
                {
                    match coin_flip() {
                        "Monster" => player.turn = Some(false),
                        _ => player.turn = Some(true),
                    }
                }

                if action == EventMenuAction::Select("Attack") {
                    combat(monster, player);
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

    pub fn clear_outcome(&mut self) {
        match self {
            EventID::MonsterEvent(monster) => monster.outcome = None,
            EventID::FairyEvent(fairy) => fairy.outcome = None,
            EventID::GenieEvent(genie) => genie.outcome = None,
            _ => {}
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
    }
}

impl Entity for Fairy {
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
        if let EventMenuAction::Select(_) = action {
            self.outcome = Some(true);

            let increase_value = choose_random_range(1..3) * (player.current_coordinate.z + 1);

            if action == EventMenuAction::Select("Increase HP") {
                let hp_percentage = player.hp as f32 / player.hp_limit as f32;
                player.hp_limit += increase_value;
                player.hp = (player.hp_limit as f32 * hp_percentage) as i32;

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
    }
}

impl Entity for Genie {
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
            outcome: None,
        }
    }

    fn penalty(&mut self, player: &mut Player, action: EventMenuAction) {
        if action == EventMenuAction::Select("Leave") && self.outcome.is_none() {
            self.outcome = Some(true);

            let penalty = self.hp / 4;
            player.hp -= penalty;
            player.money -= penalty;
            player.clip_stats();
            player.event_log.message = Some(format!("-{} to HP and Money.", penalty));
        }
    }

    fn clip_hp(&mut self) {
        self.hp = self.hp.max(0)
    }
}

impl Entity for Monster {
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

impl Attack for Monster {
    fn power(&self) -> i32 {
        choose_random_range(self.attack_power.0..self.attack_power.1 + 1)
    }
}
