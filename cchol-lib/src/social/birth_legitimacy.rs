//! 105: Illegitimacy Reasons
//! 
//! Birth (Il)legitimacy determination.
use dicebag::DiceExt;
use rpgassist::gender::HasGender;
use serde::{Deserialize, Serialize};

use crate::{Workpad, modifier::{CuMod, LegitMod}, social::{birth::Birth, culture::CultureCoreType, people::Relation}, traits::{HasCulture, HasCultureCoreType}};

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

impl LegitMod for Option<(i32, IllegitimacyReason)> {
    fn legitmod(&self) -> i32 {
        match self {
            None => 0,
            Some((legitmod,_)) => *legitmod
        }
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

pub(crate) fn determine_illegitimacy(workpad: &mut Workpad) -> Option<(i32, IllegitimacyReason)> {
    if workpad.race().lineage_strictness().illegitimacy_not_recognized_concept() {
        return None;
    }

    let roll = 1.d20();
    if (workpad.culture().core_type() == &CultureCoreType::Primitive && roll == 20)
       || (workpad.culture().core_type() != &CultureCoreType::Primitive && roll + workpad.cumod() >= 19)
    {
        return Some((1.d4(), IllegitimacyReason::random(workpad)));
    }

    None
}

#[cfg(test)]
mod birth_legitimacy_tests {
    use rpgassist::ext::IsNamed;

    use crate::{Workpad, racial::race::RACES, social::{birth_legitimacy::determine_illegitimacy, culture::CULTURES}};

    #[test]
    fn illegitimacy_does_not_apply_for_faun() {
        let mut workpad = Workpad::new();
        workpad += RACES.iter().find(|r| r.name().to_lowercase() == "faun").unwrap();
        workpad += CULTURES.iter().find(|c| c.name().to_lowercase() == "nomad").unwrap();
        if let Some(e) = determine_illegitimacy(&mut workpad) {
            panic!("Legitimacy check should be flagged off with race.lineage_strictness().illegitimacy_not_recognized_concept(), but: {e:?}")
        }
    }
}