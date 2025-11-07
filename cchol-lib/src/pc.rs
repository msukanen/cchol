use std::collections::HashMap;

use cchol_pm::{Gendered, HasName};
use rpgassist::{ext::IsNamed, gender::{Gender, HasGender}, stat::{Stat, StatBase}};
use serde::{Deserialize, Serialize};

use crate::{racial::Race, social::{culture::Culture, status::SocialStatus}};

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

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct StatMap(HashMap<StatBase, Stat>);

impl Default for StatMap {
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
} impl PlayerCharacter {
    /// Generate a random PC.
    pub fn random(name: &str) -> Self {
        let race = Race::default();
        let culture = Culture::random(race.max_culture());
        Self {
            name: name.into(),
            gender: race.random_gender(),
            stats: StatMap::default(),
            race,
            status: SocialStatus::random(&culture),
            culture
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
}