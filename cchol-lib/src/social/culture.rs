//! 102: Culture
use std::{fmt::Display, fs::{self}};

use cchol_pm::{HasCuMod, HasName, HasRollRange};
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use rpgassist::ext::IsNamed;

use crate::{serialize::{deserialize_fixed_cr_range, validate_cr_ranges, deserialize_nativeofs_to_vec}, skill::{IsLiteracySource, native_env::{IsNativeOf, NativeOf}}, roll_range::*};

/// FYI: all data files oughta reside within `./data/`.
static CULTURE_FILE: &'static str = "./data/culture.json";
lazy_static! {
    // raw json content… load/validate combo — final published product then in CULTURES below.
    static ref __CULTURES: Vec<Culture> = serde_jsonc::from_str::<Vec<Culture>>(
        &fs::read_to_string(CULTURE_FILE).expect(format!("Error with '{CULTURE_FILE}'?!").as_str())
    ).expect("JSON failure");

    /// Dice type to use for [Culture] [random][Culture::random]'izing.
    static ref CULTURE_RANGE: RollRange = validate_cr_ranges("CULTURES", &__CULTURES, None);

    /// Cultures!
    pub(crate) static ref CULTURES: Vec<Culture> = {
        let mut modded = vec![];
        __CULTURES.iter().for_each(|c|{
            match &c.native_of {
                NativeOf::Choice { primary, secondary } => {
                    modded.push(c.clone());
                    modded.push(c.clone());
                    let mut flipped = c.clone();
                    flipped.native_of = NativeOf::Choice { primary: secondary.clone(), secondary: primary.clone() };
                    modded.push(flipped);
                }
                _ => modded.push(c.clone()),
            }
        });
        modded
    };

    /// Default max [Culture] for e.g. [Race][crate::racial::Race]'s checks.
    pub(crate) static ref CULTURE_DEFAULT_MAX: &'static Culture = &CULTURES.iter()
        .find(|c| c._default_max == true)
        .expect("No default max Culture defined!");
}

/// Fixed "core types" for cultures.
#[derive(Debug, PartialEq)]
pub enum CultureCoreType {
    Primitive,
    Nomad,
    Barbarian,
    Civilized,
    Decadent
}

impl CultureCoreType {
    pub fn as_str(&self) -> &'static str {
        match self {
            CultureCoreType::Barbarian => "Barbarian",
            CultureCoreType::Civilized => "Civilized",
            CultureCoreType::Decadent => "Decadent",
            CultureCoreType::Nomad => "Nomad",
            CultureCoreType::Primitive => "Primitive",
        }
    }
}

impl Display for CultureCoreType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

pub trait HasCultureCoreType {
    fn core_type(&self) -> &'static CultureCoreType;
}

impl HasCultureCoreType for CultureCoreType {
    fn core_type(&self) -> &'static CultureCoreType {
        match self {
            Self::Barbarian => &CultureCoreType::Barbarian,
            Self::Civilized => &CultureCoreType::Civilized,
            Self::Decadent => &CultureCoreType::Decadent,
            Self::Nomad => &CultureCoreType::Nomad,
            Self::Primitive => &CultureCoreType::Primitive
        }
    }
}

/// A trait for anything that acts (or routes) a CuMod.
pub trait CuMod {
    /// Get the effective [CuMod].
    fn cumod(&self) -> i32;
}

/// Culture dwells here.
#[derive(
    Debug, Deserialize, Serialize, Clone,
    HasRollRange, HasName, HasCuMod,
)]
pub struct Culture {
    name: String,
    cumod: i32,
    /// Culture's native environment(s).
    native_of: NativeOf,
    /// CAUTION: range of roll results for randomly generating this particular [Culture].
    #[serde(deserialize_with = "deserialize_fixed_cr_range")]
    _cr_range: std::ops::RangeInclusive<i32>,
    #[serde(default)] _default_max: bool,
    #[serde(default)] provides_skills: Option<Vec<(String, i32)>>,
    #[serde(default)] literacy_chance: Option<Vec<(String, i32)>>,
    #[serde(default, deserialize_with = "deserialize_nativeofs_to_vec")]
    incompatible_env: Option<Vec<NativeOf>>,
}

impl Display for Culture {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

impl PartialEq for Culture {
    fn eq(&self, other: &Self) -> bool {
        self.cumod == other.cumod
    }
}

impl PartialOrd for Culture {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.cumod.partial_cmp(&other.cumod)
    }
}

impl IsNativeOf for Culture {
    /// Get the [Culture]'s primary [native environment][NativeOf].
    fn native_of(&self) -> NativeOf {
        self.native_of.primary()
    }
}

impl Culture {
    /// Generate a random [Culture] entry which respects given max.
    pub fn random<C: CuMod>(max_culture_mod: &C) -> &'static Culture {
        let max_cumod = max_culture_mod.cumod();
        loop {
            let candidate = Self::random_unbiased();
            if candidate.cumod() <= max_cumod {
                return candidate;
            }
        }
    }

    /// Generate a random [Culture] entry.
    pub fn random_unbiased() -> &'static Culture {
        CULTURES.get_random_in_range(&*CULTURE_RANGE)
    }

    pub fn is_civilized(&self) -> bool {
        self.name().to_ascii_lowercase() == "civilized"
    }

    pub fn is_nomad(&self) -> bool {
        self.name().to_ascii_lowercase() == "nomad"
    }

    /// Get [Culture] by name.
    /// 
    /// **FYI:** we *intentionally* panic if `value` is not found.
    pub fn from(value: &str) -> &'static Self {
        CULTURES.iter()
            .find(|c| c.name().to_lowercase() == value.to_lowercase())
            .expect(format!("No culture called '{}' found!", value).as_str())
    }

    /// Get a list of skills the [Culture] provides, if any.
    pub fn provides_skills(&self) -> Option<&Vec<(String, i32)>> {
        self.provides_skills.as_ref()
    }

    /// Check if the culture is incompatible with the given `environment`.
    pub fn incompatible_with_env(&self, environment: &NativeOf) -> bool {
        self.incompatible_env.as_ref()
            .is_some_and(|e| e.contains(environment))
    }
}

impl IsLiteracySource for Culture {
    fn literacy_skills(&self) -> Vec<(String, i32)> {
        self.literacy_chance.as_ref().cloned().unwrap_or_default()
    }
}

impl HasCultureCoreType for Culture {
    fn core_type(&self) -> &'static CultureCoreType {
        match self.name().to_lowercase().as_str() {
            "primitive" => &CultureCoreType::Primitive,
            "nomad" => &CultureCoreType::Nomad,
            "barbarian" => &CultureCoreType::Barbarian,
            "civilized" => &CultureCoreType::Civilized,
            "decadent" => &CultureCoreType::Decadent,
            _ => unimplemented!("No core type determinable for '{}'", self.name())
        }
    }
}

#[cfg(test)]
/// Note that these tests work correctly with the shipped data files.
/// Altering the data files too much may or may not break one or the other test case...
/// 
/// Notably the tests rely on presence of "Primitive", "Nomad", "Barbarian", "Civilized", and "Decadent",
/// of which "Decadent" oughta be the pinnacle max in what comes to culture levels.
mod culture_tests {
    use super::*;

    #[test]
    fn simple_cr_range_read_from_json() {
        let prjson = r#"{
            "name": "Primitive",
            "cumod": -3,
            "native_of": "Wilderness",
            "_cr_range": 1
        }"#;
        let prc: Culture = serde_jsonc::from_str(prjson).unwrap();
        assert_eq!("Primitive", prc.name());
        assert_eq!(-3, prc.cumod());
        assert_eq!(NativeOf::Wilderness, prc.native_of());
    }

    #[test]
    fn complex_cr_range_read_from_json() {
        let prjson = r#"{
            "name": "Barbarian",
            "cumod": 2,
            "native_of": {
                "primary": "wilderness",
                "secondary": "urban"
            },
            "_cr_range": [2,3]
        }"#;
        let prc: Culture = serde_jsonc::from_str(prjson).unwrap();
        assert_eq!("Barbarian", prc.name());
        assert_eq!(2, prc.cumod());
        assert_eq!(NativeOf::Wilderness, prc.native_of());
        assert_eq!(NativeOf::Urban, prc.native_of().opposite());
    }

    #[test]
    fn spam_data_integrity() {
        assert_eq!(5, CULTURES.len());
        (0..=1000).for_each(|_| {
            let c = Culture::random_unbiased();
            assert!(["Primitive","Barbarian","Nomad","Civilized","Decadent"].contains(&c.name()));
        });
    }

    #[test]
    fn default_max_is_decadent() {
        assert_eq!("Decadent", CULTURE_DEFAULT_MAX.name());
    }

    #[test]
    fn culture_random_respects_max() {
        let rounds = 1001;
        let _ = env_logger::try_init();
        let maxc = Culture::from("barbarian");
        let mut suitable_found = 0;
        (0..rounds).for_each(|_| {
            let c = Culture::random(maxc);
            assert!(c.cumod() <= maxc.cumod());
            if c.cumod() <= maxc.cumod() {
                suitable_found += 1;
            }
        });
        assert_eq!(rounds, suitable_found);
    }

    #[test]
    fn primitive_is_incompatible_with_urban() {
        let env = NativeOf::Urban;
        let c = CULTURES.iter().find(|c| c.name().to_lowercase() == "primitive").unwrap();
        assert!(c.incompatible_with_env(&env))
    }

    #[test]
    fn bogus_is_incompatible_with_many() {
        let bogus_culture = r#"{
            "name": "Primitive",
            "cumod": -3,
            "native_of": "Wilderness",
            "_cr_range": 1,
            "provides_skills": [["Survival: Wilderness", 5], ["Survival: Urban", 1]],
            "literacy_chance": [["Other Culture", 5]],
            "incompatible_env": ["Urban", "Air"]
        }"#;
        let c: Culture = serde_jsonc::from_str(bogus_culture).unwrap();
        assert!(c.incompatible_with_env(&NativeOf::Air));
        assert!(c.incompatible_with_env(&NativeOf::Urban));
    }
}