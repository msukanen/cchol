//! 101: Race
use std::fs;

use cchol_pm::HasRollRange;
use lazy_static::lazy_static;
use rpgassist::{gender::{Gender, GenderBias, HasGenderBias}, ext::IsNamed};
use serde::{Deserialize, Deserializer, Serialize, de};

use crate::{events::RacialEvent, roll_range::*, serialize::{default_pc_save_cr_range, deserialize_fixed_cr_range, validate_cr_ranges}, social::{culture::{CULTURE_DEFAULT_MAX, CULTURES, CuMod, Culture}, nobility::Noble, status::SocialStatus}};

static RACE_FILE: &'static str = "./data/race.json";
lazy_static! {
    pub(crate) static ref RACES: Vec<Race> = {
        let races: Vec<Race> = serde_jsonc::from_str::<Vec<Race>>(
            &fs::read_to_string(RACE_FILE)
                .expect(format!("No '{}' found?!", RACE_FILE).as_str())
        ).expect("JSON fail!");

        let mut something_failed = false;
        for r in &races {
            if r.roll_range() == &(0..=0) {
                something_failed = true;
                log::error!("DATA VALIDATION: Race '{}' is MISSING its '_cr_range' field in '{RACE_FILE}'!", r.name());
            }
        }
        if something_failed {panic!("Cannot continue before someone fixes JSON in '{RACE_FILE}'…")}

        races
    };

    /// The 'default' race to use when non-random race is required, which
    /// usually is "human" but can be defined to be whatever else is present.
    static ref RACE_DEFAULT: &'static Race = {
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
    static ref RACE_RANGE: RollRange = validate_cr_ranges("RACES", &RACES, None);

    /// Dice range for [Race] [random][Race::random]'izing in non-human range.
    static ref RACE_RANGE_NONHUMAN: RollRange = {
        let human_end = RACES.iter().find(|r| r.name().to_lowercase() == "human").unwrap().roll_range().end();
        (*human_end + 1)..=(*RACE_RANGE.end() as i32)
    };

    /// If access to race specs of specifically "human" is required…
    static ref RACE_HUMAN: &'static Race = {
        RACES.iter().find(|r|r.name().to_lowercase() == "human").expect("Where'd the hummies go?!")
    };
}

/// Deserialize a [Race]'s max culture, if any.
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
#[derive(Debug, Deserialize, Serialize, Clone, HasRollRange)]
pub struct Race {
    /// Name of the race, obviously.
    name: String,
    /// The race's max [Culture] level.
    #[serde(
        rename = "max_culture",
        deserialize_with = "deserialize_race_max_culture",
        skip_serializing,
        default
    )]
    max_culture: Option<&'static Culture>,
    /// ...roll range for [`Race::random`]...
    #[serde(
        rename = "_cr_range",
        deserialize_with = "deserialize_fixed_cr_range",
        skip_serializing,
        default = "default_pc_save_cr_range"
    )]
    _cr_range: std::ops::RangeInclusive<i32>,
    #[serde(default)] hybrid: bool,
    /// INIT-ONLY flag for being a default race when non-random is requested...
    #[serde(skip_serializing, default)] _default: Option<bool>,
    #[serde(skip_serializing, default)] shift_nomad_down: bool,
    #[serde(skip_serializing, default)] shift_civilized_up: bool,
    #[serde(default)] racial_events: Option<RacialEvent>,
    #[serde(default)] hybrid_events: Option<RacialEvent>,
    #[serde(skip_serializing, default)] gender_bias: GenderBias,
    #[serde(default)] beastman: bool,
    #[serde(default)] reptilian: bool,
    #[serde(default, skip_serializing)] forced_gender: Option<Gender>,
    #[serde(default, skip_serializing)] convert_title: Vec<(String, String)>,
}

impl HasGenderBias for Race {
    fn gender_bias(&self) -> GenderBias {
        self.gender_bias
    }
}

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

    /// Get a random [Race].
    pub fn random() -> &'static Race {
        RACES.get_random_in_range(&*RACE_RANGE)
    }

    /// Get a random non-human [Race].
    pub fn random_nonhuman() -> &'static Race {
        RACES.get_random_in_range(&*RACE_RANGE_NONHUMAN)
    }

    /// Shift culture one way or the other if the given `culture` doesn't
    /// comply with the [Race]'s requirements.
    /// 
    /// # Returns
    /// **a)** a clone of `culture` itself, or
    /// **b)** a shifted [Culture] instance.
    pub fn shift_culture_if_needed(&self, culture: &'static Culture) -> &'static Culture {
        if self.max_culture() < &culture {
            return self.max_culture();
        }

        if !self.shift_civilized_up && !self.shift_nomad_down {
            return culture;
        }
        else if self.shift_civilized_up && culture.is_civilized() {
            if let Some(higher_candidate) = CULTURES.iter()
                    .find(|c| c.cumod() > culture.cumod()) {
                return higher_candidate;
            }
        }
        else if self.shift_nomad_down && culture.is_nomad() {
            if let Some(lower_candidate) = CULTURES
                    .iter()
                    .rev() // to find the nearest smaller-than. Without .rev() we'd always hit rock bottom instead.
                    .find(|c| c.cumod() < culture.cumod())
            {
                return lower_candidate;
            }
        }

        log::debug!("No shifting required");
        culture
    }

    /// Get a race by `name`.
    /// 
    /// **FYI:** We *intentionally* panic if `value` is not found.
    pub fn from(name: &str) -> &'static Self {
        RACES.iter()
            .find(|r| r.name().to_lowercase() == name.to_lowercase())
            .expect(format!("No race called '{name}' found!").as_str())
    }

    /// Get a race by (optional) `name`.
    /// 
    /// If no name is provided (`None`), then we hand out the default race.
    pub fn from_opt(name: Option<String>) -> &'static Self {
        if let Some(name) = name {
            Self::from(name.as_str())
        } else {
            Self::random()
        }
    }

    /// Hand out a static ref to the default [Race].
    pub fn default() -> &'static Self {
        &RACE_DEFAULT
    }

    /// See if the [Race] is a human+other hybrid.
    pub fn is_hybrid(&self) -> bool {
        self.hybrid
    }

    /// See if the [Race] is a beastman species.
    pub fn is_beastman(&self) -> bool {
        self.beastman
    }

    /// See if the [Race] is reptilian/batrachian species.
    pub fn is_reptilian(&self) -> bool {
        self.reptilian
    }

    /// Get [RacialEvents] table, if any.
    pub fn has_racial_events(&self, raised_by_humans: bool) -> Option<RacialEvent> {
        if let Some(e) = &self.racial_events {
            return Some(e.clone())
        }

        if !raised_by_humans {
            if let Some(e) = &self.hybrid_events {
                return Some(e.clone())
            }
        }

        None
    }

    /// Race does at times affect gender distribution, hence…
    pub fn random_gender(&self) -> Gender {
        if let Some(fg) = &self.forced_gender {
            return *fg;
        }
        Gender::random_biased(self.gender_bias)
    }

    /// Adjust `gender` to conform with the [Race] specs, if needed.
    /// 
    /// Generally does nothing except when given `gender` that is
    /// not compatible with the [Race] specs.
    pub fn adjust_gender(&self, gender: Gender) -> Gender {
        if let Some(fg) = &self.forced_gender {
            if gender != *fg {
                return *fg
            }
        }
        gender
    }

    /// Adjust social status to conform with [Race] specs, if needed.
    pub fn adjust_social_status(&self, ss: SocialStatus) -> SocialStatus {
        if self.convert_title.is_empty() { return ss;}
        
        if let Some(n) = ss.nobility() {
            // adjust noble title if needed…
            for (from_title, to_title) in &self.convert_title {
                if n.name().to_lowercase() == from_title.to_lowercase() {
                    let nobility = Some(Noble { name: (to_title.into(), None), ..(*n).clone() });
                    return SocialStatus { nobility, ..ss };
                }
            }
        }
        
        ss
    }
}