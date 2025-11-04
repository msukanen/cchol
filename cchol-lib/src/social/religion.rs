//! 864: Deities

use std::fs;

use cchol_pm::{Gendered, HasRollRange};
use dicebag::DiceExt;
use lazy_static::lazy_static;
use rpgassist::{gender::{Gender, GenderBias, HasGender}, resolve::resolve_in_place::ResolveInPlace};
use serde::{Deserialize, Serialize};
use crate::{modifier::CuMod, roll_range::{UseRollRange, RollRange}, serialize::{deserialize_strings_to_vec, deserialize_cr_range}, traits::personality::{AffectsAlignment, Alignment}};

/// Deity "alignments".
#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum DeityAlignment {
    Benign,
    Neutral,
    Evil,
    Decadent,
    DisguisedEvil,
} impl Default for DeityAlignment {
    fn default() -> Self {
        Self::Neutral
    }
} impl AffectsAlignment for DeityAlignment {
    fn alignment(&self) -> Alignment {
        match self {
            Self::Benign   => Alignment::L,
            Self::Neutral  => Alignment::N,
            Self::Decadent => Alignment::N,
            Self::Evil |
            Self::DisguisedEvil => Alignment::D
        }
    }
}

static DEITY_FILE: &'static str = "./data/deity.json";
lazy_static! {
    static ref DEITIES: Vec<Deity> = serde_jsonc::from_str(
        &fs::read_to_string(DEITY_FILE).expect(format!("Error with '{DEITY_FILE}'?!").as_str())
    ).expect("JSON error");
}

#[derive(Debug, Deserialize, Serialize, Clone, HasRollRange, Gendered)]
pub struct Deity {
    name: String,
    #[serde(default, deserialize_with = "deserialize_strings_to_vec")]
    alt: Vec<String>,
    #[serde(default)]
    gender: Gender,
    #[serde(default, skip_serializing)]
    gender_bias: GenderBias,
    #[serde(default)]
    alignment: DeityAlignment,
    #[serde(deserialize_with = "deserialize_cr_range")]
    _cr_range: RollRange,
} impl Deity {
    /// Get a random [Deity].
    pub fn random(cumod_src: &impl CuMod) -> Self {
        let mut alignment = DeityAlignment::Neutral;
        let mut deity = loop {
            let roll = 1.d20() + cumod_src.cumod();
            match roll {
                15 => {
                    alignment = DeityAlignment::Evil;
                    continue;
                },
                25.. => {
                    alignment = match alignment {
                        DeityAlignment::Decadent |
                        DeityAlignment::Evil     => DeityAlignment::DisguisedEvil,
                        _ => DeityAlignment::Decadent,
                    };
                    continue;
                }
                _ => break DEITIES.iter()
                    .find(|d| d.roll_range().contains(&roll))
                    .expect(format!("DATA VALIDITY: missing _cr_range for '{roll}'").as_str())
            }
        }.clone();
        deity.resolve();
        deity.alignment = alignment;
        deity
    }
}

impl ResolveInPlace for Deity {
    fn resolve(&mut self) {
        self.gender.resolve_biased(self.gender_bias);
        let roll = 1.d(self.alt.len() + 1) as i32 - 2;
        if roll >= 0 {
            self.name = self.alt[roll as usize].clone()
        }
    }
}