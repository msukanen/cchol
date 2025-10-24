//! 104: Birth
//! 105: Illegitimate Births
//! 108: Birth Order
use dicebag::DiceExt;
use either::Either;
use rpgassist::{body::birthmark::Birthmark, gender::Gender};
use serde::{Deserialize, Serialize};

use crate::{misc::{benefit::Benefit, time_of_year::TimeOfBirth, TimeOfYear}, modifier::{BiMod, CuMod, LegitMod}, social::{culture::CultureLevelType, family::FamilyMember}};

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

/// Generate date (or season) of birth.
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

/// Generate count of unusual birth events, if any.
/// 
/// # Args
/// `bimod_src`— some [BiMod] source.
/// 
/// # Returns
/// Either a single value or a tuple: `(num-player-choice, num-GM-choice)`.
fn determine_number_of_unusual_birth_events(bimod_src: &impl BiMod) -> Either<i32, (i32, i32)> {
    match 1.d100() + bimod_src.bimod() {
        ..=60 => Either::Left(0),
        ..=76 => Either::Left(1),
        ..=85 => Either::Left(2),
        ..=92 => Either::Right((1,1)),
        ..=94 => Either::Left(3),
        ..=97 => {
            let gm_num = 1.d2();
            let pc_num = 3 - gm_num;
            Either::Right((pc_num, gm_num))
        },
        98 => Either::Left(4),
        _ => {
            let gm_num = 1.d3();
            let pc_num = 4 - gm_num;
            Either::Right((pc_num, gm_num))
        }
    }
}

/// Determine unusual birth (or not-so-unusual…).
/// 
/// # Args
/// `bimod_src`— some [BiMod] source.
pub fn determine_unusual_birth(bimod_src: &impl BiMod) -> Option<Vec<UnusualBirthCircumstance>> {
    let occurances = match determine_number_of_unusual_birth_events(bimod_src) {
        Either::Left(n) => n,
        Either::Right((a, b)) => a + b
    };
}

/// Sub-choices for multiselects…
#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum UBCSubChoice {
    I { benefits: Vec<Benefit> },
    II { benefits: Vec<Benefit> },
    III { benefits: Vec<Benefit> },
    IV { benefits: Vec<Benefit> },
    V { benefits: Vec<Benefit> },
    VI { benefits: Vec<Benefit> },
    VII { benefits: Vec<Benefit> },
    VIII { benefits: Vec<Benefit> },
    IX { benefits: Vec<Benefit> },
    X { benefits: Vec<Benefit> },
}

/// Various unusual birth circumstances.
#[derive(Debug, Deserialize, Serialize, Clone)]
//TODO: flesh out descriptions etc.
pub enum UnusualBirthCircumstance {
    Combined((Box<UnusualBirthCircumstance>, Box<UnusualBirthCircumstance>)),
    Ubc001005 { blamed: bool, reincarnation: bool },
    Ubc006010,
    Ubc011020,
    Ubc021023,
    Ubc024025,
    Ubc026027,
    Ubc028031 { separated: bool, different_personalities: bool },
    Ubc032034,
    Ubc035037,
    Ubc038___,
    Ubc039041 { subchoices: Vec<UBCSubChoice>},
    Ubc042044 { subchoices: Vec<UBCSubChoice>},
    Ubc045___,// NOTE: missing in sourcebook!
    //Ubc046048( Curse ),//TODO T-868
    Ubc049050 { rating: u8 },// Rating 1-10; 7-9 = still have, 10 = have and it's magical.
    Ubc051053,
    Ubc054055,
    Ubc056___,
    Ubc057___,
    Ubc058062,
    //Ubc063064( Tragedy ),//TODO T-528
    Ubc065069( Birthmark ),
    //Ubc070075( Curse ),//TODO T-868
    //Ubc076081( Blessing ),//TODO T-869
    Ubc082085 { gender: Gender },
    Ubc086___,
    //Ubc087088( DeathSituation ),//TODO T-545
    //Ubc089093( PhysicalAffliction ),//TODO T-874
    //Ubc094___( PsychicAbility ),//TODO T-873
    //Ubc095099( GiftNLegacy ),//TODO T-863
    Ubc101105,
    Ubc106110 {
        str: Benefit,
        dex: Benefit,
        int: Benefit,
        con: Benefit,
        mag: Benefit,
        app: Benefit,
        cha: Benefit,
        //physical_affliction: PhysicalAffliction,//TODO T-874
        //curse: Curse,//TODO T-868
        //trait: DarksideTrait,// TODO T-648
    },
}