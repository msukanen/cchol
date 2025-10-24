//! 103: Social Status (Wealth part)
use dicebag::DiceExt;
use rpgassist::modifier::HasModifier;
use serde::{Deserialize, Serialize};

use crate::{modifier::{CuMod, SolMod, SurvivalMod, WealthMod}, social::nobility::Nobility};

/// Being 'wealthy' comes with distinct levels of 'wealthy'.
#[derive(Debug, Deserialize, Serialize, PartialEq, PartialOrd, Eq, Ord)]
pub enum RichesMagnitude {
    Wealthy,
    VeryWealthy,
    Rich,
    FilthyRich,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, PartialOrd)]
/// Various wealth levels.
pub enum WealthLevel {
    Destitute { survival_mod: i32 },
    Poor,
    Comfortable,
    WellToDo,
    Wealthy { magnitude: RichesMagnitude, survival_mod: i32 },
}

impl SolMod for WealthLevel {
    /// Wealth portion of **SolMod**.
    fn solmod(&self) -> i32 {
        match self {
            Self::Destitute {..} => -3,
            Self::Poor => -1,
            Self::Comfortable => 0,
            Self::WellToDo => 2,
            Self::Wealthy { magnitude: RichesMagnitude::Wealthy, .. } => 4,
            Self::Wealthy { .. } => 8,
        }
    }
}

impl WealthLevel {
    /// Generate random wealth level.
    /// 
    /// Generally used internally by e.g. [SocialStatus::new()][`crate::society::SocialStatus::new`],
    /// but can be called as-is for simpler matters.
    /// 
    /// # Args
    /// `cumod_src`— some [CuMod] source.
    pub fn new(cumod_src: &impl CuMod, nobility: Option<&Nobility>) -> Self {
        fn mk_level(cumod: i32, nobility: Option<&Nobility>) -> WealthLevel {
            match 1.d100() + cumod {
                ..=12 => WealthLevel::Destitute { survival_mod: 1.d2() },
                ..=40 => WealthLevel::Poor,
                ..=84 => WealthLevel::Comfortable,
                85 => mk_level(0, nobility),
                ..=94 => WealthLevel::WellToDo,
                ..=98 => WealthLevel::Wealthy {
                    magnitude:
                        if 1.d100() <= (
                            1 + if let Some(nobility) = nobility.cloned()
                                     {nobility.modifier()}
                                else {0}
                            ){RichesMagnitude::VeryWealthy}
                        else {RichesMagnitude::Wealthy},
                    survival_mod: -(1.d2())
                },
                _ => mk_level(cumod, nobility)// this would be "nobility", but that's handled elsewhere…
            }
        }

        mk_level(cumod_src.cumod(), nobility)
    }

    /// Step wealth level up by one rank, if possible.
    pub fn rankup(self) -> WealthLevel {
        match &self {
            Self::Destitute { .. } => Self::Poor,
            Self::Poor => Self::Comfortable,
            Self::Comfortable => Self::WellToDo,
            Self::WellToDo => Self::Wealthy { magnitude: RichesMagnitude::Wealthy, survival_mod: 0 },
            Self::Wealthy { magnitude, survival_mod } => match magnitude {
                RichesMagnitude::Wealthy => Self::Wealthy { magnitude: RichesMagnitude::VeryWealthy, survival_mod: *survival_mod },
                RichesMagnitude::VeryWealthy => Self::Wealthy { magnitude: RichesMagnitude::Rich, survival_mod: *survival_mod },
                RichesMagnitude::Rich => Self::Wealthy { magnitude: RichesMagnitude::FilthyRich, survival_mod: *survival_mod },
                RichesMagnitude::FilthyRich => {
                    log::info!("Already 'filthy rich', there's no higher magnitude defined.");
                    self
                }
            }
        }
    }

    /// Step wealth level down by one rank, if possible.
    pub fn rankdown(self) -> WealthLevel {
        match &self {
            Self::Destitute { .. } => {
                log::info!("Already 'destitute'. One cannot go below having nothing.");
                self
            },
            Self::Poor => Self::Destitute { survival_mod: 1.d2() },
            Self::Comfortable => Self::Poor,
            Self::WellToDo => Self::Comfortable,
            Self::Wealthy { magnitude, survival_mod } => match magnitude {
                RichesMagnitude::Wealthy => Self::WellToDo,
                RichesMagnitude::VeryWealthy => Self::Wealthy { magnitude: RichesMagnitude::Wealthy, survival_mod: *survival_mod },
                RichesMagnitude::Rich => Self::Wealthy { magnitude: RichesMagnitude::VeryWealthy, survival_mod: *survival_mod },
                RichesMagnitude::FilthyRich => Self::Wealthy { magnitude: RichesMagnitude::Rich, survival_mod: *survival_mod },
            }
        }
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

impl WealthMod for WealthLevel {
    fn wmod(&self) -> f64 {
        match self {
            Self::Destitute { .. } => 0.25,
            Self::Poor => 0.5,
            Self::Comfortable => 1.0,
            Self::WellToDo => 2.0, //NOTE: Well-to-Do in CCHoL has just 150%, but we use 200% here.
            Self::Wealthy { magnitude, .. } =>
                match magnitude {
                    RichesMagnitude::Wealthy => 5.0,
                    RichesMagnitude::VeryWealthy => 10.0,
                    RichesMagnitude::Rich => 20.0,
                    RichesMagnitude::FilthyRich => 50.0,
                }
        }
    }
}