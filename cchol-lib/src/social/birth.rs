//! 104: Birth
//! 
//! Anything and everything related to
//!   time of birth,
//!   place of birth,
//!   unusual birth circumstances,
//! etc.
use dicebag::{DiceExt, IsOne};
use rpgassist::gender::Gender;
use serde::{Deserialize, Serialize};

use crate::{places::birthplace::PlaceOfBirth, racial::Race, social::{SolMod, birth_legitimacy::{IllegitimacyReason, LegitMod, SiblingLegit, determine_illegitimacy}, culture::Culture, family::FamilyStructure, people::relative::RelationSubType}};
/// A trait for anything that gives out **BiMod**.
pub trait BiMod {
    /// Get **BiMod** (birth modifier).
    fn bimod(&self) -> i32;
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Birth {
    pub(in crate::social) illegitimacy_info: Option<(i32, IllegitimacyReason)>,
    //place_of_birth: PlaceOfBirth,
    siblings: Vec<SiblingLegit>,
    family: FamilyStructure,
    birth_order: BirthOrder,
    place_of_birth: PlaceOfBirth,
}

impl BiMod for Birth {
    fn bimod(&self) -> i32 {
        self.place_of_birth.bimod()
    }
}

impl SolMod for Birth {
    fn solmod(&self) -> i32 {
        -(self.legitmod())
    }
}

/// Birth order — dictactes a number of things from starting money to inheritance, etc.
#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum BirthOrder {
    First,// used also for "only child"
    Second,
    LastOf3,
    Middle,
    LastOf4,
    SecondToLast,
    Last
} impl BirthOrder {
    /// Get birth order related starting money modifier.
    fn starting_money_mod(&self) -> f64 {
        match self {
            Self::First => 1.0,
            Self::Second => 0.9,
            Self::Middle  |
            Self::LastOf3 => 0.85,
            Self::SecondToLast |
            Self::LastOf4 => 0.80,
            Self::Last => 0.75
        }
    }

    /// Generate random birth order based on number of siblings (if any).
    fn random(num_siblings: usize) -> Self {
        match num_siblings {
            0 => Self::default(),
            1 => if 1.d2().is_one() {Self::First} else {Self::Second},
            2 => match 1.d3() {
                1 => Self::First,
                2 => Self::Second,
                _ => Self::LastOf3
            },
            3 => match 1.d6() {
                1|2 => Self::First,
                3 => Self::Second,
                4|5 => Self::Middle,
                _ => Self::LastOf4
            },
            _ => match 1.d10() {
                1 => Self::First,
                2|3 => Self::Second,
                4..=6 => Self::Middle,
                7|8 => Self::SecondToLast,
                _ => Self::Last
            }
        }
    }
} impl Default for BirthOrder {
    /// Get somewhat sensible default as [BirthOrder].
    fn default() -> Self {
        Self::First
    }
}

impl Birth {
    pub fn random(gender: &Gender, race: &'static Race, culture: &'static Culture) -> Self {
        let illegitimacy_info = determine_illegitimacy(race, culture);
        let family = FamilyStructure::random(culture);
        let siblings = {
            let mut siblings = vec![];
            let mut i = 1;
            while i > 0 {
                i -= 1;
                let (il, le) = match 1.d20() {
                    ..=2 => break,
                    ..=9 => (0, 1.d3()),
                    ..=15 => (0, 1.d3()+1),
                    ..=17 => (0, 1.d4()+2),
                    ..=19 => (0, 2.d4()),
                    _ => {
                        i += 1;
                        (1.d3(), 0)
                    }
                };
                (0..il).for_each(|_| siblings.push(SiblingLegit::Illegit(RelationSubType::random_sibling(race))));
                (0..le).for_each(|_| siblings.push(SiblingLegit::Legit(RelationSubType::random_sibling(race))));
            }
            siblings
        };

        Self {
            family,
            birth_order: BirthOrder::random(siblings.len()),
            siblings,
            place_of_birth: PlaceOfBirth::random(race, culture, &illegitimacy_info),
            illegitimacy_info,
        }
    }

    pub fn starting_money_mod(&self) -> f64 {
        self.birth_order.starting_money_mod()
    }
}