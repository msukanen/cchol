//! 105: Illegitimacy Reasons
//! 
//! Birth (Il)legitimacy determination.
use dicebag::DiceExt;
use rpgassist::gender::HasGender;
use serde::{Deserialize, Serialize};

use crate::{racial::Race, social::{CuMod, birth::Birth, culture::{Culture, CultureCoreType, HasCultureCoreType}, people::Relation}};

/// A trait for anything that delivers birth related **LegitMod**.
pub trait LegitMod {
    /// Get the **LegitMod**..
    fn legitmod(&self) -> i32;
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum SiblingLegit {
    Legit(Relation),
    Illegit(Relation),
} impl HasGender for SiblingLegit {
    fn gender(&self) -> rpgassist::gender::Gender {
        // just pass thro…
        match self {
            Self::Illegit(r)|
            Self::Legit(r)  => r.gender()
        }
    }
}

impl LegitMod for Birth {
    /// Get the **LegitMod**.
    /// 
    /// Note: **LegitMod** has (or should have) a direct negative impact on a character's **SolMod**
    /// (except when said **SolMod** is negative to begin with).
    /// 
    /// Note: if a [Noble][crate::social::nobility::Noble] born has **LegitMod**,
    /// they are not eligible for any inheritance unless they are the sole heir to begin with.
    fn legitmod(&self) -> i32 {
        if let Some((legitmod,_)) = self.illegitimacy_info {
            return legitmod;
        }
        0 // nothing to report here - tots legit birth.
    }
}

/// Some reasons for possible illegitimacy of birth.
#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum IllegitimacyReason {
    MotherCommonPrositute,
    MotherRaped { father_known: bool },
    MotherUnmarried { father_known: bool },
    MotherCourtesan { father_known: bool },
} impl IllegitimacyReason {
    pub fn random(cumod_src: &impl CuMod) -> Self {
        match 1.d20() + cumod_src.cumod() {
            ..=12 => Self::MotherCommonPrositute,
            ..=14 => Self::MotherRaped { father_known: 1.d100() < 16 },
            ..=23 => Self::MotherUnmarried { father_known: 1.d100() < 51 },
            _ => Self::MotherCourtesan { father_known: 1.d100() < 51 }
        }
    }
}

pub(crate) fn determine_illegitimacy(
        race: &'static Race,
        culture: &'static Culture
) -> Option<(i32, IllegitimacyReason)> {
    let roll = 1.d20();
    if (culture.core_type() == &CultureCoreType::Primitive && roll == 20)
       || (culture.core_type() != &CultureCoreType::Primitive && roll + culture.cumod() >= 19)
    {
        return Some((1.d4(), IllegitimacyReason::random(culture)));
    }

    None
}