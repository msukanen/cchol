//! 110: Place of Birth
use dicebag::DiceExt;
use serde::{Deserialize, Serialize};

use crate::{misc::time_period::TimePeriod, modifier::LegitMod, racial::race::Race};

/// Some places of birth.
#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum PlaceOfBirth {
    InFamilyHome,
    InHospitalOrHealerGuild,
    InCarriage,
    InCommonBarn,
    InForeignLand(Box<PlaceOfBirth>),
    InCave,
    InMiddleOfField,
    InForest,
    Exotic(ExoticPlaceOfBirth),
}

impl PlaceOfBirth {
    /// Generate a random place of birth.
    pub fn new(legit: &impl LegitMod) -> Self {
        match 1.d20() + legit.legitmod() {
            ..=6 => Self::InFamilyHome,
            ..=9 => Self::InHospitalOrHealerGuild,
            10 => Self::InCarriage,
            11 => Self::InCommonBarn,
            ..=13 => Self::InForeignLand(Box::new(Self::new(legit))),
            14 => Self::InCave,
            15 => Self::InMiddleOfField,
            16 => Self::InForest,
            _ => Self::Exotic(ExoticPlaceOfBirth::new())
        }
    }
}

/// Somewhat exotic places of birth.
#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum ExoticPlaceOfBirth {
    Combined(Box<ExoticPlaceOfBirth>, Box<ExoticPlaceOfBirth>),
    InTempleOfGood,//TODO: T-864.
    OnBattlefield { among_camp_followers: bool },
    InAlley,
    InBrothel { mother_courtesan: bool },
    InPalaceOfLocalRuler,
    InPalaceOfCountryRuler,
    InPalaceOfGreatEvil,
    /// Bar, tavern, alehouse, etc.
    InTavern,
    InThievesDen,
    InNonhumansHome { race: Race },
    //GM(GMOnly) //TODO: 978#111
    InSewers,
    InTempleOfEvil,
    AnotherPlaneOfReality,
    AnotherTimePeriod { from: TimePeriod },
    OnShipAtSea,
    InPrisonCell { mother_imprisoned: bool },
    InWizardLab,
}

impl ExoticPlaceOfBirth {
    /// Generate a random exotic place of birth.
    pub(super) fn new() -> Self {
        match 1.d20() {
            ..=2 => Self::Combined(Box::new(Self::new()), Box::new(Self::new())),
            3 => Self::InTempleOfGood,
            4 => Self::OnBattlefield { among_camp_followers: 1.d6() < 6 },
            5 => Self::InAlley,
            6 => Self::InBrothel { mother_courtesan: 1.d6() == 6 },
            7 => Self::InPalaceOfLocalRuler,
            8 => Self::InPalaceOfCountryRuler,
            9 => Self::InPalaceOfGreatEvil,
            10 => Self::InTavern,
            11 => Self::InSewers,
            12 => Self::InThievesDen,
            13 => Self::InNonhumansHome { race: Race::new_nonhuman() },
            14 => unimplemented!("Self::GM(GMOnly::new(GM_111))"),
            15 => Self::InTempleOfEvil,
            16 => Self::AnotherPlaneOfReality,
            17 => Self::AnotherTimePeriod { from: TimePeriod::new_nonpresent() },
            18 => Self::OnShipAtSea,
            19 => Self::InPrisonCell { mother_imprisoned: 1.d3() == 1 },
            _ => Self::InWizardLab
        }
    }
}