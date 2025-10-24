//! 104: Birth
//! 105: Illegitimate Births
//! 108: Birth Order
use dicebag::DiceExt;
use serde::{Deserialize, Serialize};

use crate::{misc::{time_of_year::TimeOfBirth, TimeOfYear}, modifier::{CuMod, LegitMod}, society::{culture::CultureLevelType, family::FamilyMember}};

#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
pub enum BirthLegitimacy {
    Legitimate,
    Illegitimate(IllegitimacyReason),
}

impl BirthLegitimacy {
    pub fn is_legitimate(&self) -> bool {
        match self {
            Self::Legitimate => true,
            _ => false
        }
    }
}

impl LegitMod for BirthLegitimacy {
    fn legitmod(&self) -> i32 {
        match self {
            Self::Legitimate => 0,
            Self::Illegitimate(r) => r.legitmod()
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
/// Some illegitimacy reasons.
//TODO: flesh out actually verbose.
pub enum IllegitimacyReason {
    R1(i32),
    R2 { legit_mod: i32, father_known: bool },
    R3 { legit_mod: i32, father_known: bool },
    R4 { legit_mod: i32, father_known: bool },
}

impl LegitMod for IllegitimacyReason {
    fn legitmod(&self) -> i32 {
        match self {
            Self::R1(m) => *m,
            Self::R2 { legit_mod, ..}|
            Self::R3 { legit_mod, ..}|
            Self::R4 { legit_mod, ..} => *legit_mod
        }
    }
}

/// Determine birth (il)legitimacy.
/// 
/// # Args
/// `cumod_src`— some [CuMod] source.
pub fn determine_birth_legitimacy(cumod_src: &impl CuMod) -> BirthLegitimacy {
    // Generate a culturally biased illegitimacy reason.
    fn mk_illegitimacy_reason(cumod_src: &impl CuMod) -> BirthLegitimacy {
        BirthLegitimacy::Illegitimate(match 1.d20() + cumod_src.cumod() {
            ..=12 => IllegitimacyReason::R1(1.d4()),
            ..=14 => IllegitimacyReason::R2 { legit_mod: 1.d4(), father_known: 1.d100() < 16 },
            ..=23 => IllegitimacyReason::R3 { legit_mod: 1.d4(), father_known: 1.d100() < 51 },
            _     => IllegitimacyReason::R4 { legit_mod: 1.d4(), father_known: 1.d100() < 51 },
        })
    }

    match cumod_src.as_clt() {
        CultureLevelType::Primitive => if 1.d20() == 20 { mk_illegitimacy_reason(cumod_src)} else { BirthLegitimacy::Legitimate },
        _ => if 1.d20() + cumod_src.cumod() > 19 { mk_illegitimacy_reason(cumod_src)} else { BirthLegitimacy::Legitimate },
    }
}

/// Various birth orders, from last to first.
#[derive(Debug, Deserialize, Serialize, PartialEq, PartialOrd)]
pub enum BirthOrder {
    LastBorn,
    SecondToTheLastBorn,
    MiddleChild,
    SecondBorn,
    FirstBorn,
}

/// Determine birth order. Largely irrelevant if there is no siblings…
/// 
/// # Args
/// `siblings`— optional siblings vec.
pub fn determine_birth_order(siblings: Option<&Vec<FamilyMember>>) -> Option<BirthOrder> {
    //TODO: some refinement to order selection. Base on actual number of siblings perhaps?
    if let Some(_) = siblings {
        return Some(match 1.d20() {
            ..=2 => BirthOrder::FirstBorn,
            ..=10 => BirthOrder::SecondBorn,
            ..=16 => BirthOrder::MiddleChild,
            ..=18 => BirthOrder::SecondToTheLastBorn,
            _ => BirthOrder::LastBorn
        });
    }
    None
}

/// Date of birth.
/// 
/// # Args
/// `birth_legit`— birth was legitimate?
/// 
/// # Returns
/// Either day + month or just (an approximate) time of year, depending on whether
/// birth was legit or not.
pub fn determine_date_of_birth(birth_legit: &BirthLegitimacy) -> TimeOfBirth {
    if birth_legit.is_legitimate() {
        TimeOfBirth::Date { day: 1.d(30), month: 1.d12() }
    } else {
        TimeOfBirth::Approximate(TimeOfYear::new())
    }
}