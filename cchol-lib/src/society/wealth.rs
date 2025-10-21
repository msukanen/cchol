//! 103: Social Status (Wealth part)
use dicebag::DiceExt;
use rpgassist::modifier::HasModifier;
use serde::{Deserialize, Serialize};

use crate::{modifier::{CuMod, SolMod, SurvivalMod}, society::{culture::CultureLevelType, nobility::Nobility}};

#[derive(Debug, Deserialize, Serialize, PartialEq, PartialOrd)]
pub enum WealthLevel {
    Destitute { survival_mod: i32 },
    Poor,
    Comfortable,
    WellToDo,
    Wealthy { extremely: bool, survival_mod: i32 },
}

impl SolMod for WealthLevel {
    /// Wealth portion of **SolMod**.
    fn solmod(&self) -> i32 {
        match self {
            Self::Destitute {..} => -3,
            Self::Poor => -1,
            Self::Comfortable => 0,
            Self::WellToDo => 2,
            Self::Wealthy { extremely: false,.. } => 4,
            Self::Wealthy { .. } => 8,
        }
    }
}

impl WealthLevel {
    /// Generate random wealth level.
    /// 
    /// Generally used internally by e.g. [SocialStatus::new()][`crate::society::SocialStatus::new`],
    /// but can be called as-is for simpler matters.
    pub fn new(culture_type: &CultureLevelType, nobility: Option<&Nobility>) -> Self {
        fn mk_level(cumod: i32, nobility: Option<&Nobility>) -> WealthLevel {
            match 1.d100() + cumod {
                ..=12 => WealthLevel::Destitute { survival_mod: 1.d2() },
                ..=40 => WealthLevel::Poor,
                ..=84 => WealthLevel::Comfortable,
                85 => mk_level(0, nobility),
                ..=94 => WealthLevel::WellToDo,
                ..=98 => WealthLevel::Wealthy {
                    extremely: 1.d100() <= 1 + if let Some(nobility) = nobility.cloned()
                            {nobility.modifier()} else {0},
                    survival_mod: -(1.d2())
                },
                _ => mk_level(cumod, nobility)// this would be "nobility", but that's handled elsewhere…
            }
        }

        mk_level(culture_type.cumod(), nobility)
    }
}

impl SurvivalMod for WealthLevel {
    fn survmod(&self) -> i32 {
        match self {
            Self::Destitute { survival_mod } => *survival_mod,
            Self::Poor        =>  1,
            Self::Comfortable =>  0,
            Self::WellToDo    => -1,
            Self::Wealthy { survival_mod,.. } => *survival_mod,
        }
    }
}