use std::fs;

use dicebag::DiceExt;
use lazy_static::lazy_static;
use serde::{de, Deserialize, Deserializer};

use crate::{default_roll_range_def, serialize::deserialize_cr_range, social::culture::{CuMod, Culture, CULTURES, CULTURE_DEFAULT_MAX}, traits::HasRollRange, IsNamed};

static RACE_FILE: &'static str = "./data/race.json";
lazy_static! {
    static ref RACES: Vec<Race> = {
        serde_jsonc::from_str::<Vec<Race>>(
            &fs::read_to_string(RACE_FILE)
                .expect(format!("No '{}' found?!", RACE_FILE).as_str())
        ).expect("JSON fail!")
    };

    pub static ref RACE_DEFAULT: &'static Race = {
        let defaults: Vec<&'static Race> = RACES.iter()
            .filter(|r| r.is_default())
            .collect();
        match defaults.len() {
            0 => panic!("DATA VALIDATION: no default Race specified!"),
            1 => defaults[0],
            _ => panic!("DATA VALIDATION: \"There can be only one!\" - which isn't true in this case. Too many defaults ({}) defined!", defaults.len())
        }
    };

    /// Dice type to use for [Race] [random][Race::random]'izing.
    static ref RACE_DICE: usize = crate::create_dice_size!(RACES);
}

fn deserialize_race_max_culture<'de,D>(deserializer: D) -> Result<Option<&'static Culture>, D::Error>
where D: Deserializer<'de> {
    let maybe_name = Option::<String>::deserialize(deserializer)?;

    if let Some(name) = maybe_name {
        CULTURES.iter()
            .find(|c| c.name() == name)
            .map(Some)
            .ok_or_else(|| {
                de::Error::custom(format!(
                    "DATA ERROR: no Culture found with name '{}'",
                    name
                ))
            })
    } else {
        Ok(None)
    }
}

/// Race specs.
#[derive(Debug, Deserialize, Clone)]
pub struct Race {
    /// Name of the race, obviously.
    name: String,
    /// The race's max [Culture] level.
    #[serde(
        rename = "max_culture",
        deserialize_with = "deserialize_race_max_culture",
        default
    )]
    max_culture: Option<&'static Culture>,
    /// ...roll range for [`Race::random`]...
    #[serde(
        rename = "_cr_range",
        deserialize_with = "deserialize_cr_range"
    )]
    _cr_range: std::ops::RangeInclusive<i32>,
    /// INIT-ONLY flag for being a default race when non-random is requested...
    #[serde(default)] _default: Option<bool>,
    #[serde(default)] shift_nomad_down: bool,
    #[serde(default)] shift_civilized_up: bool,
}

default_roll_range_def!(Race);

impl IsNamed for Race {
    fn name(&self) -> &str {
        &self.name
    }
}

impl Race {
    /// Get max. [Culture] of the [Race].
    pub fn max_culture(&self) -> &'static Culture {
        if let Some(mx) = self.max_culture {
            mx
        } else {
            &CULTURE_DEFAULT_MAX
        }
    }

    /// Is this race the default when non-random species is requested?
    fn is_default(&self) -> bool {
        self._default.unwrap_or_default()
    }

    pub fn random() -> &'static Race {
        let roll = 1.d(*RACE_DICE);
        RACES.iter()
            .find(|r| r.roll_range().contains(&roll))
            .expect("Something went awefully wronk here…")
    }

    pub fn shift_culture_if_needed(&self, culture: &'static Culture) -> &'static Culture {
        if self.max_culture() < culture {
            return self.max_culture();
        }

        if !self.shift_civilized_up && !self.shift_nomad_down {
            return culture;
        }

        if self.shift_civilized_up && culture.is_civilized() {
            if let Some(higher_candidate) = CULTURES.iter()
                    .find(|c| c.cumod() > culture.cumod()) {
                return higher_candidate;
            }
        } else if self.shift_nomad_down && culture.is_nomad() {
            if let Some(lower_candidate) = CULTURES.iter().find(|c| c.cumod() < culture.cumod()) {
                return lower_candidate;
            }
        }

        log::debug!("No shifting required");
        culture
    }

    /// Get a race by name.
    /// 
    /// **FYI:** We *intentionally* panic if `value` is not found.
    pub fn from(value: &str) -> &'static Self {
        RACES.iter()
            .find(|r| r.name().to_lowercase() == value.to_lowercase())
            .expect(format!("No race called '{}' found!", value).as_str())
    }
}