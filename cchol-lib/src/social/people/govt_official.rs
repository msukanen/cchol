use std::fs;

use cchol_pm::{Gendered, HasRollRange};
use dicebag::DiceExt;
use lazy_static::lazy_static;
use rpgassist::{gender::{Gender, GenderBias, HasGender}, resolve::resolve_in_place::ResolveInPlace, serialize::serial_strings::deserialize_strings_to_vec};
use serde::{Deserialize, Serialize};

use crate::{serialize::{default_pc_save_cr_range, deserialize_fixed_cr_range, validate_cr_ranges}, roll_range::*};

fn govt_alt_default() -> Vec<String> {vec![]}

static GOVT_OFFICIALS_FILE: &'static str = "./data/people_govtoff.json";
lazy_static! {
    static ref GOVT_OFFICIALS: Vec<GovtOfficial> = {
        let goffs: Vec<GovtOfficial> = serde_jsonc::from_str(
            &fs::read_to_string(GOVT_OFFICIALS_FILE)
                .expect(format!("File error with '{GOVT_OFFICIALS_FILE}'!").as_str())
        ).expect("JSON error");
        goffs
    };

    static ref GOVT_RANGE: std::ops::RangeInclusive<i32> = validate_cr_ranges("GOVT_OFFICIALS", &GOVT_OFFICIALS, None);
}

#[derive(Debug, Deserialize, Serialize, Clone, HasRollRange, Gendered)]
pub struct GovtOfficial {
    name: String,
    #[serde(deserialize_with = "deserialize_strings_to_vec", default = "govt_alt_default", skip_serializing)]
    alt: Vec<String>,
    #[serde(deserialize_with = "deserialize_fixed_cr_range", default = "default_pc_save_cr_range", skip_serializing)]
    _cr_range: std::ops::RangeInclusive<i32>,
    #[serde(default)]
    gender: Gender
}

impl ResolveInPlace for GovtOfficial {
    fn resolve(&mut self) {
        self.gender.resolve_biased(GenderBias::Male23);
        if self.alt.len() > 0 {
            let roll = 1.d(self.alt.len() + 1);
            if roll > 1 {
                self.name = self.alt[roll - 2 as usize].clone()
            }
        }
    }
}

/// Generate a random govt official.
pub fn random() -> GovtOfficial {
    let mut goff = GOVT_OFFICIALS.get_random_in_range(&GOVT_RANGE).clone();
    goff.resolve();
    goff
}

#[cfg(test)]
mod govt_official_tests {
    use super::*;

    #[test]
    fn people_govtoff_json_data_integrity() {
        let goff = random();
        assert_ne!(Gender::Unspecified, goff.gender());
    }

    #[test]
    fn get_rwr() {
        let mut goff = GOVT_OFFICIALS.get_random_in_range(&GOVT_RANGE).clone();
        goff.resolve();
        assert_ne!(Gender::Unspecified, goff.gender())
    }
}