//! 103: Social Status (Wealth)

use std::fs;

use lazy_static::lazy_static;

use rpgassist::{resolve::resolve_in_place::ResolveInPlace, ext::IsNamed};
use serde::{Deserialize, Serialize};
use dicebag::{DiceExt, DiceT};

use crate::{modifier::CuMod, roll_range::*, serialize::{deserialize_dicet, deserialize_optional_cr_range}};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Wealth {
    name: String,
    #[serde(default)] aliases: Option<Vec<String>>,
    #[serde(default)] solmod: i32,
    #[serde(
        rename = "survival_mod",
        deserialize_with = "deserialize_dicet",
        default
    )]  survival_mod: DiceT,
    /// CAUTION: range of roll results for randomly generating this particular [Wealth].
    #[serde(
        rename = "_cr_range",
        deserialize_with = "deserialize_optional_cr_range",
        default
    )]  _cr_range: Option<std::ops::RangeInclusive<i32>>,
}

impl IsNamed for Wealth {
    fn name(&self) -> &str {
        &self.name
    }
}

impl Wealth {
    /// Generate random [Wealth] level.
    pub fn random(cumod_src: &impl CuMod) -> &'static Self {
        fn mk_wealth(cumod: i32) -> &'static Wealth {
            let roll = 1.d100() + cumod;
            if roll == 85 { return mk_wealth(0) }
            if let Some(w) = WEALTH.iter().find(|x| x.roll_range().contains(&roll)) {
                return w;
            }
            mk_wealth(cumod)
        }

        mk_wealth(cumod_src.cumod())
    }
}

impl ResolveInPlace for Wealth {
    /// **NOTE:** the resolver is to be used on a **.clone()**'d [Wealth] instance.
    fn resolve(&mut self) {
        let smod = self.survival_mod.0.d(self.survival_mod.1 as usize);
        self.survival_mod = (smod, 1);
    }
}

/// A range that is nigh impossible to roll with dice. Used for stuff that needs _cr_range to be present but are not in basic roll tables.
static NO_RANGE: std::ops::RangeInclusive<i32> = i32::MIN..=i32::MIN;
impl UseRollRange for Wealth {
    fn roll_range(&self) -> &std::ops::RangeInclusive<i32> {
        self._cr_range.as_ref().unwrap_or_else(|| &NO_RANGE)
    }
}

static WEALTH_FILE: &'static str = "./data/wealth.json";
lazy_static! {
    static ref WEALTH: Vec<Wealth> = {
        serde_jsonc::from_str(
            &fs::read_to_string(WEALTH_FILE)
                .expect(format!("No '{}' found?", WEALTH_FILE).as_str())
        ).expect("JSON failed")
    };
}

#[cfg(test)]
mod wealth_data_integrity {
    use super::*;

    static SPAM_RANGE: std::ops::RangeInclusive<i32> = 0..=1000;

    #[test]
    fn wealth_file_parses() {
        let _ = env_logger::try_init();
        
        // The must-be-present wealth levels…
        let tof : [&str;6] = ["Destitute", "Poor", "Comfortable", "Well-to-Do", "Wealthy", "Extremely Wealthy"];
        
        // See that there is *at least* a required number of wealth levels present…
        assert!(tof.len() <= WEALTH.len());

        // See that each of the required ones actually exist…
        tof.iter().for_each(|name|{
            if let Some(_) = WEALTH.iter().find(|w| w.name() == *name) {
                log::info!("OK, found '{name}'")
            } else {
                panic!("'{name}' not listed?!")
            }
        });
    }

    #[test]
    fn survival_mod_resolve() {
        let _ = env_logger::try_init();
        for _ in SPAM_RANGE.clone() {
            let mut w = WEALTH.iter().find(|w| w.name().to_lowercase() == "destitute").unwrap().clone();
            assert!(w.survival_mod.0 == 1 || w.survival_mod.0 == 2);
            assert_eq!(w.survival_mod.1, 1);
        }
    }
}