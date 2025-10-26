//! 102: Culture
use std::{fs::{self}, ops::RangeInclusive};

use dicebag::DiceExt;
use lazy_static::lazy_static;
use serde::Deserialize;

use crate::{skill::native_env::{IsNativeOf, NativeOf}, traits::HasRollRange, Named, serialize::deserialize_cr_range};

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

static CULTURE_FILE: &'static str = "./data/culture.json";
lazy_static! {
    pub(crate) static ref CULTURES: Vec<Culture> = {
        let cultures = serde_jsonc::from_str::<Vec<Culture>>(
            &fs::read_to_string(CULTURE_FILE).expect(
                format!("We got '{}', but could not read from it… What gives?", CULTURE_FILE).as_str()
            )
        ).expect("JSON failure");

        validate_culture_ranges(&cultures);

        cultures
    };
    pub(crate) static ref CULTURE_DICE: usize = {
        CULTURES.iter()
            .map(|c| *c.roll_range().end())
            .max()
            .expect("CULTURES list is empty?!")
            as usize
    };
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
}

impl CuMod for Culture {
    fn cumod(&self) -> i32 {
        self.cumod
    }
}

impl Named for Culture {
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

impl HasRollRange for Culture {
    fn roll_range(&self) -> &std::ops::RangeInclusive<i32> {
        &self._cr_range
    }
}

impl Culture {
    pub fn random() -> Self {
        let roll = 1.d(*CULTURE_DICE);
        for c in CULTURES.iter() {
            if c.roll_range().contains(&roll) {
                return c.clone();
            }
        }
        
        // Should not happen, but…
        panic!("Roll of {roll} didn't catch ANY Culture?!")
    }
}

#[cfg(test)]
mod culture_tests {
    use super::*;

    #[test]
    fn simple_read_from_json() {
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
    fn complex_read_from_json() {
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
            let c = Culture::random();
            assert!(["Primitive","Barbarian","Nomad","Civilized","Decadent"].contains(&c.name()));
        });
    }
}