//! 101: Race, 751: Nonhumans
use dicebag::DiceExt;
use rpgassist::gender::Gender;
use serde::{Deserialize, Serialize};

use crate::society::culture::{CultureLevelType, Level};

/// Various (playable) races.
#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub enum Race {
    Human,
    Elf,
    Dwarf,
    Halfling,
    /// Half-breed between two different species.
    Hybrid {
        /// Hybrid was raised by 0/left, 1/right, of the "halves".
        raised_by: u8,
        /// Parent species 0 and 1.
        halves: (Box<Race>, Box<Race>),
    },
    Centaur,
    Minotaur,
    Satyr,
    Faun,
    Goatman,
    Reptileman,
    Serpentman,
    Dragonman,
    Orc,
}

impl Race {
    /// Is the race a "beastman" species?
    pub fn is_beastman(&self) -> bool {
        match self {
            Self::Centaur |
            Self::Minotaur |
            Self::Satyr |
            Self::Faun |
            Self::Goatman => true,
            _ => false
        }
    }

    /// Is the race considered reptilian-like?
    pub fn is_reptilian(&self) -> bool {
        match self {
            Self::Reptileman |
            Self::Serpentman |
            Self::Dragonman => true,
            _ => false
        }
    }

    /// Is the race actually a hybrid?
    pub fn is_hybrid(&self) -> bool {
        match self {
            Self::Hybrid { .. } => true,
            _ => false
        }
    }

    /// Raised by …
    pub fn raised_by(&self) -> Self {
        match self {
            Race::Hybrid { raised_by, halves } => if *raised_by == 0 { *halves.0.clone() } else { *halves.1.clone() },
            _ => self.clone()
        }
    }

    /// Get max culture level, if any exist.
    pub fn max_culture(&self) -> Option<CultureLevelType> {
        match self.raised_by() {
            Self::Human |
            Self::Elf => None,
            Self::Hybrid { raised_by, halves } => if raised_by == 0 && *halves.0 == Self::Human {None} else { halves.1.max_culture() },
            Self::Dwarf |
            Self::Halfling => Some(CultureLevelType::Civilized),
            Self::Orc => Some(CultureLevelType::Barbarian),
            r => if r.is_beastman() {
                if r == Self::Centaur { Some(CultureLevelType::Civilized) } else { Some(CultureLevelType::Barbarian) }
            } else if r.is_reptilian() {
                None
            } else {
                //FYI: a catch all for unspecified entries.
                Some(CultureLevelType::Barbarian)
            }
        }
    }

    /// Shift `culture` one way or the other if race so requires.
    ///
    /// Non-reptiles keep whatever culture they have while [reptilemen][`Race::Reptileman`]
    /// shift [nomads][`Level::Nomad`] into [primitives][`Level::Primitive`]
    /// and more [civilized][`Level::Civilized`] into [decadency][`Level::Decadent`].
    ///
    pub fn culture_shift_if_needed(&self, culture: Level) -> Level {
        if self.is_reptilian() {
            match CultureLevelType::from(culture.clone()) {
                CultureLevelType::Nomad => Level::Primitive,
                CultureLevelType::Civilized => Level::Decadent,
                _ => culture
            }
        } else {
            culture
        }
    }

    /// Generate a random race.
    // T-101
    pub fn new() -> Self {
        match 1.d20() {
            ..=14 => Self::Human,
            _ => Self::new_nonhuman()
        }
    }

    /// Generate a random non-human race.
    // T-751
    pub fn new_nonhuman() -> Self {
        match 1.d20() {
            ..=4 => Self::Elf,
            ..=8 => Self::Dwarf,
            ..=11 => Self::Halfling,
            ..=15 => Self::mk_humanhybrid(Race::Elf),
            ..=16 => Self::new_beastman(),
            17 => Self::new_reptile(),
            18 => Self::Orc,
            _ => Self::mk_humanhybrid(Race::Orc),
        }
    }

    fn mk_humanhybrid(other: Race) -> Self {
        let raised_by = (1_i32.d2() - 1) as u8;
        Self::Hybrid { raised_by, halves: (Box::new(Race::Human), Box::new(other)) }
    }

    /// Readjust given gender to conform with race's requirements. Some races have
    /// strictly fixed gender as it is.
    pub fn readjust_gender(&self, gender: Gender) -> Gender {
        let new_gender = match self {
            Self::Faun => Gender::Female,
            Self::Goatman |
            Self::Satyr => Gender::Male,
            _ => gender.clone()
        };
        if new_gender.ne(&gender) {
            log::info!("Race enforced genderbend from '{:?}' to '{:?}'.", gender, new_gender)
        }
        new_gender
    }

    /// Generate a random beastman race.
    pub fn new_beastman() -> Self {
        match 1.d8() {
            ..=3 => Self::Centaur,
            4|5 => Self::Faun,
            6|7 => if 1.d3() < 3 {Self::Satyr} else {Self::Goatman},
            _ => Self::Minotaur,
        }
    }

    /// Generate a random reptile race.
    pub fn new_reptile() -> Self {
        match 1.d6() {
            ..=3 => Self::Reptileman,
            4|5 => Self::Serpentman,
            _ => Self::Dragonman
        }
    }

    /// Generate random gender, with or without species-specific gender bias.
    pub fn rnd_gender(&self) -> Gender {
        match self {
            Self::Minotaur |
            Self::Goatman  |
            Self::Satyr    => Gender::Male,
            Self::Faun     => Gender::Female,
            _ => Gender::new(match self {
            Self::Human    |
            Self::Halfling |
            Self::Reptileman|
            Self::Serpentman|
            Self::Dragonman|
            Self::Hybrid { .. }|
            Self::Elf      => None,
            Self::Dwarf    |
            Self::Centaur  |
            Self::Orc      => Some(Gender::Male),
            _              => unimplemented!("Not implemented for Gender::rnd_gender({:?})!", self)
            })
        }
    }
}

impl From<&str> for Race {
    fn from(value: &str) -> Self {
        match value.to_lowercase().as_str() {
            "centaur" => Race::Centaur,
            "dragonman" => Race::Dragonman,
            "dwarf" => Race::Dwarf,
            "goatman" => Race::Goatman,
            "elf" => Race::Elf,
            "faun" => Race::Faun,
            "halfelf" => Race::mk_humanhybrid(Race::Elf),
            "halfling" => Race::Halfling,
            "halforc" => Race::mk_humanhybrid(Race::Orc),
            "human" => Race::Human,
            "minotaur" => Race::Minotaur,
            "reptileman" => Race::Reptileman,
            "satyr" => Race::Satyr,
            "serpentman" => Race::Serpentman,
            _ => unimplemented!("Race '{value}' has not been implemented. It might be in the future, but for now…")
        }
    }
}

impl From<Option<String>> for Race {
    fn from(value: Option<String>) -> Self {
        if let Some(racename) = value {
            Self::from(racename.as_str())
        } else {
            Self::new()
        }
    }
}