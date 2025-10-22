//! 104: Birth

use dicebag::DiceExt;
use serde::{Deserialize, Serialize};

use crate::{modifier::{CuMod, LegitMod}, society::culture::CultureLevelType};

#[derive(Debug, Deserialize, Serialize)]
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
/// Birth is considered legit if `None` is returned as a reason.
/// 
/// # Args
/// `cumod_src`— some [CuMod] source.
pub fn determine_birth_illegitimacy(cumod_src: &impl CuMod) -> Option<IllegitimacyReason> {
    // Generate a culturally biased illegitimacy reason.
    fn mk_illegitimacy_reason(cumod_src: &impl CuMod) -> IllegitimacyReason {
        match 1.d20() + cumod_src.cumod() {
            ..=12 => IllegitimacyReason::R1(1.d4()),
            ..=14 => IllegitimacyReason::R2 { legit_mod: 1.d4(), father_known: 1.d100() < 16 },
            ..=23 => IllegitimacyReason::R3 { legit_mod: 1.d4(), father_known: 1.d100() < 51 },
            _     => IllegitimacyReason::R4 { legit_mod: 1.d4(), father_known: 1.d100() < 51 },
        }
    }

    match cumod_src.as_clt() {
        CultureLevelType::Primitive => if 1.d20() == 20 { Some(mk_illegitimacy_reason(cumod_src))} else { None },
        _ => if 1.d20() + cumod_src.cumod() > 19 { Some(mk_illegitimacy_reason(cumod_src))} else { None },
    }
}
