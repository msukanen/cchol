use std::fs;

use cchol_pm::{Gendered, HasRollRange};
use dicebag::DiceExt;
use lazy_static::lazy_static;
use rpgassist::gender::{Gender, HasGender};
use serde::{Deserialize, Serialize};

use crate::{IsNamed, racial::Race, roll_range::*, serialize::{deserialize_cr_range, validate_cr_ranges}};

static ADVENTURER_FILE: &'static str = "./data/adventurer.json";
lazy_static! {
    static ref ADVENTURERS: Vec<Adventurer> = serde_jsonc::from_str(
        &fs::read_to_string(ADVENTURER_FILE).expect(format!("Error with '{ADVENTURER_FILE}'?!").as_str())
    ).expect("JSON error");

    static ref ADVENTURERS_RANGE: RollRange = validate_cr_ranges("ADVENTURERS", &ADVENTURERS, None);
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum AdventurerProwess {
    NotApplicable,
    Beginner { skill_rank: u8 },
    Veteran { skill_rank: u8},
    LocalHero { skill_rank: u8 },
    GrandMaster { skill_rank: u8 }
} impl Default for AdventurerProwess {
    fn default() -> Self {
        Self::NotApplicable
    }
} impl AdventurerProwess {
    fn random() -> Self {
        match 1.d20() {
            ..=5 => Self::Beginner { skill_rank: 1.d3() },
            ..=14 => Self::Veteran { skill_rank: 1.d2() + 3 },
            ..=19 => Self::LocalHero { skill_rank: 1.d3() + 5 },
            _ => Self::GrandMaster { skill_rank: match 1.d20() {
                ..=10 => 8,
                ..=17 => 9,
                ..=19 => 10,
                _ => if 1.d3() == 1 {11} else {10}
            } }
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, HasRollRange, Gendered)]
pub struct Adventurer {
    name: String,
    #[serde(deserialize_with = "deserialize_cr_range")]
    _cr_range: RollRange,
    #[serde(default)]// race is meaningful only for (N)PC instances.
    race: Option<String>,
    #[serde(default)]// gender is meaningful only for (N)PC instances.
    gender: Gender,
    #[serde(default)]
    prowess: AdventurerProwess,
} impl Adventurer {
    /// Generate a random [Adventurer].
    pub fn random() -> Self {
        let adv = ADVENTURERS.get_random_in_range(&ADVENTURERS_RANGE);
        // a non-human adventurer maybe?
        let race = if 1.d20() == 1 {
            Some(Race::random_nonhuman().name())
        } else { None };
        Self {
            race: race.map(|s| s.into()),
            gender: Gender::random(),
            prowess: AdventurerProwess::random(),
            .. adv.clone()
        }
    }
}

impl IsNamed for Adventurer {
    fn name(&self) -> &str {
        &self.name
    }
}

#[cfg(test)]
mod adventurer_tests {
    use crate::social::people::adventurer::ADVENTURERS;

    #[test]
    fn adventurer_file_data_integrity() {
        assert!(9 <= ADVENTURERS.len());
    }
}