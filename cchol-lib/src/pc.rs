//! # **Player Character**
use std::collections::HashMap;

use cchol_pm::{Gendered, HasName};
use rpgassist::{ext::IsNamed, gender::{Gender, HasGender}, stat::{Stat, StatBase}, serialize::serial_uf64::deserialize as uf64_deserialize};
use serde::{Deserialize, Serialize};

use crate::{racial::Race, social::{birth::Birth, culture::Culture, status::SocialStatus}};

/// Default starting money, be it $, €, credits, gold, or something else.
static DEFAULT_STARTING_MONEY: f64 = 1_000.0;

/// (De)serializer for PC save [Race] entry.
mod serial_pc_race {
    use rpgassist::ext::IsNamed;
    use serde::{Deserialize, Deserializer, Serializer};

    use crate::racial::{Race, race::RACES};

    pub(super) fn deserialize<'de,D>(deserializer: D) -> Result<&'static Race, D::Error>
    where D: Deserializer<'de> {
        let race_name = String::deserialize(deserializer)?;
        // fail-fast if exact name match (case ignorant) isn't found…!
        Ok(&*RACES.iter()
            .find(|n| n.name().to_lowercase() == race_name.to_lowercase())
            .expect(format!("SAVE FILE: non-existent Race '{race_name}' defined!").as_str()))
    }

    pub(super) fn serialize<S>(race: &&'static Race, serializer: S) -> Result<S::Ok, S::Error>
    where S: Serializer {
        serializer.serialize_str(race.name())
    }
}

/// (De)serialized for PC save [Culture] entry.
mod serial_pc_culture {
    use rpgassist::ext::IsNamed;
    use serde::{Deserialize, Deserializer, Serializer};

    use crate::social::culture::{CULTURES, Culture};

    pub(super) fn deserialize<'de,D>(deserializer: D) -> Result<&'static Culture, D::Error>
    where D: Deserializer<'de> {
        let cult_name = String::deserialize(deserializer)?;
        // fail-fast if exact name match (case ignorant) isn't found…!
        Ok(&*CULTURES.iter()
            .find(|n| n.name().to_lowercase() == cult_name.to_lowercase())
            .expect(format!("SAVE FILE: non-existent Culture '{cult_name}' defined!").as_str()))
    }

    pub(super) fn serialize<S>(culture: &&'static Culture, serializer: S) -> Result<S::Ok, S::Error>
    where S: Serializer {
        serializer.serialize_str(culture.name())
    }
}

/// [Stat] map for e.g. [PlayerCharacter] etc.
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct StatMap(HashMap<StatBase, Stat>);

impl Default for StatMap {
    /// Generate default [StatMap] with more or less sensible default values for each stat present.
    fn default() -> Self {
        let mut map = HashMap::new();
        map.insert(StatBase::Age, StatBase::Age.default());
        map.insert(StatBase::App, StatBase::App.default());
        map.insert(StatBase::Cha, StatBase::Cha.default());
        map.insert(StatBase::Con, StatBase::Con.default());
        map.insert(StatBase::Dex, StatBase::Dex.default());
        map.insert(StatBase::Int, StatBase::Int.default());
        map.insert(StatBase::Mag, StatBase::Mag.default());
        map.insert(StatBase::Str, StatBase::Str.default());
        map.insert(StatBase::Will, StatBase::Will.default());
        Self(map)
    }
}

fn get_starting_money_default() -> f64 {DEFAULT_STARTING_MONEY}

/// Player Character data lives here, obviously(?)…
#[derive(Debug, Deserialize, Serialize, Clone, HasName, Gendered)]
pub struct PlayerCharacter {
    name: String,
    gender: Gender,
    #[serde(with = "serial_pc_race")]
    race: &'static Race,
    stats: StatMap,
    #[serde(with = "serial_pc_culture")]
    culture: &'static Culture,
    status: SocialStatus,
    #[serde(deserialize_with = "uf64_deserialize", default = "get_starting_money_default")]
    starting_money: f64,
    birth: Birth,
} impl PlayerCharacter {
    /// Generate a random PC.
    pub fn random(name: &str) -> Self {
        let race = Race::default();
        let culture = Culture::random(race.max_culture());
        let gender = race.random_gender();
        Self {
            name: name.into(),
            stats: StatMap::default(),
            status: SocialStatus::random(&culture),
            starting_money: DEFAULT_STARTING_MONEY,
            birth: Birth::random(&gender, race, culture),

            gender, race, culture,
        }
    }

    /// Builder for specific [`gender`][Gender].
    /// 
    /// Chainable in any order.
    pub fn with_gender(&mut self, gender: Gender) -> &mut Self {
        self.gender = self.race.adjust_gender(gender);
        self
    }

    /// Builder for specific [`race`][Race].
    /// 
    /// Chainable in any order.
    pub fn with_race(&mut self, race: &'static Race) -> &mut Self {
        self.race = race;
        self.gender = self.race.adjust_gender(self.gender);
        self.culture = self.race.shift_culture_if_needed(&self.culture);
        if !self.status.is_compatible_with(&self.culture) {
            self.status = SocialStatus::random(&self.culture);
        }
        self
    }

    /// Builder for specific [`culture`][Culture].
    /// 
    /// Chainable in any order.
    pub fn with_culture(&mut self, culture: &'static Culture) -> &mut Self {
        self.culture = culture;
        self.culture = self.race.shift_culture_if_needed(&self.culture);
        if !self.status.is_compatible_with(&self.culture) {
            self.status = SocialStatus::random(&self.culture);
        }
        self
    }

    /// Builder for non-default base starting money.
    pub fn with_starting_money(&mut self, base_money: f64) -> &mut Self {
        self.starting_money = base_money.max(0.0);// 0.0 minimum so we don't not start with debt…
        self
    }

    /// See how much moneys the character has… at start.
    pub fn starting_money(&self) -> f64 {
        self.starting_money * self.status.wealth().starting_money_mod()
    }
}