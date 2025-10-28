//! 102: Culture
use std::{fs::{self}, ops::RangeInclusive};

use dicebag::DiceExt;
use lazy_static::lazy_static;
use serde::Deserialize;

use crate::{default_roll_range_def, serialize::deserialize_cr_range, skill::native_env::{IsNativeOf, NativeOf}, traits::HasRollRange, IsNamed};

fn validate_culture_ranges(cultures: &Vec<Culture>) {
    let mut ranges: Vec<&RangeInclusive<i32>> = cultures
        .iter()
        .map(|c| c.roll_range())
        .collect();
    
    ranges.sort_by(|a,b| a.start().cmp(b.start()));
    
    if ranges.is_empty() {
        panic!("DATA VALIDATION: CULTURES list is empty. Cannot validate ranges.");
    }

    if *ranges[0].start() != 1 {
        panic!("DATA VALIDATION: Culture roll table must start at 1. Found {:#?}", ranges[0]);
    }

    // Check for gaps/overlaps
    for w in ranges.windows(2) {
        let c = w[0];
        let n = w[1];
        let expected_next_start = *c.end() + 1;
        if *n.start() != expected_next_start {
            panic!("DATA VALIDATION: Gap or overlap in Culture roll table!\nFound {:#?}, followed by {:#?}", c, n);
        }
    }

    log::debug!("Culture ranges successfully validated: 1..={}", *ranges.last().unwrap().end());
}

/// FYI: all data files oughta reside within `./data/`.
static CULTURE_FILE: &'static str = "./data/culture.json";
lazy_static! {
    /// Cultures!
    pub(crate) static ref CULTURES: Vec<Culture> = {
        let cultures = serde_jsonc::from_str::<Vec<Culture>>(
            &fs::read_to_string(CULTURE_FILE)
                .expect(format!("No '{}' found?!", CULTURE_FILE).as_str())
        ).expect("JSON failure");

        validate_culture_ranges(&cultures);

        cultures
    };

    /// Dice type to use for [Culture] [random][Culture::random]'izing.
    static ref CULTURE_DICE: usize = crate::create_dice_size!(CULTURES);

    /// Default max [Culture] for e.g. [Race][crate::racial::Race]'s checks.
    pub(crate) static ref CULTURE_DEFAULT_MAX: &'static Culture = &CULTURES.iter()
        .find(|c| c._default_max == true)
        .expect("No default max Culture defined!");
}

/// A trait for anything that acts (or routes) a CuMod.
pub trait CuMod {
    /// Get the effective [CuMod].
    fn cumod(&self) -> i32;
}

/// Culture dwells here.
#[derive(Debug, Deserialize, Clone)]
pub struct Culture {
    name: String,
    cumod: i32,
    /// Culture's native environment(s).
    native_of: NativeOf,
    /// CAUTION: range of roll results for randomly generating this particular [Culture].
    #[serde(
        rename = "_cr_range",
        deserialize_with = "deserialize_cr_range"
    )]
    _cr_range: std::ops::RangeInclusive<i32>,
    #[serde(default)] _default_max: bool,
    #[serde(default)] provides_skills: Option<Vec<(String, i32)>>,
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

impl CuMod for Culture {
    fn cumod(&self) -> i32 {
        self.cumod
    }
}

impl IsNamed for Culture {
    fn name(&self) -> &str {
        &self.name
    }
}

impl IsNativeOf for Culture {
    /// Get the [Culture]'s primary [native environment][NativeOf].
    fn native_of(&self) -> NativeOf {
        self.native_of.primary()
    }
}

default_roll_range_def!(Culture);

impl Culture {
    /// Generate a random [Culture] entry which respects given max.
    pub fn random<C: CuMod>(max_culture_mod: &C) -> Culture {
        let max_cumod = max_culture_mod.cumod();
        loop {
            let candidate = Self::random_unbiased();
            if candidate.cumod() <= max_cumod {
                #[cfg(test)]{
                    log::debug!("Candidate {:?} accepted", candidate.name())
                }
                if let NativeOf::Choice { primary, secondary } = &candidate.native_of {
                    // To spice things up a bit, potentially swap primary/secondary…
                    if 1.d3() == 1 {
                        return Self { native_of: NativeOf::Choice { primary: secondary.clone(), secondary: primary.clone() }, ..candidate.clone() }
                    }
                }
                
                return candidate.clone();
            }
            #[cfg(test)] {
                log::debug!("Entry exceeded max_cumod, rerolling...")
            }
        }
    }

    /// Generate a random [Culture] entry.
    pub fn random_unbiased() -> &'static Culture {
        let roll = 1.d(*CULTURE_DICE);
        CULTURES.iter()
                .find(|c| c.roll_range().contains(&roll))
                .unwrap_or_else(|| panic!("Roll of {roll} didn't catch ANY Culture?!"))
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
    fn dryrun_load_file() {
        assert_eq!(10, *CULTURE_DICE);
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
}