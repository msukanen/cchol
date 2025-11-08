//! 104: Birth
//! 
//! Anything and everything related to
//!   time of birth,
//!   place of birth,
//!   unusual birth circumstances,
//! etc.

use rpgassist::gender::Gender;
use serde::{Deserialize, Serialize};

use crate::{racial::Race, social::{SolMod, birth_legitimacy::{IllegitimacyReason, LegitMod, SiblingLegit, determine_illegitimacy}, culture::Culture, family::{self, FamilyStructure}, people::Relation}};
/// A trait for anything that gives out **BiMod**.
pub trait BiMod {
    /// Get **BiMod** (birth modifier).
    fn bimod(&self) -> i32;
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Birth {
    pub(in crate::social) illegitimacy_info: Option<(i32, IllegitimacyReason)>,
    place_of_birth: PlaceOfBirth,
    siblings: Vec<SiblingLegit>,
    family: FamilyStructure,
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

impl Birth {
    pub fn random(gender: &Gender, race: &'static Race, culture: &'static Culture) -> Self {
        let illegitimacy_info = determine_illegitimacy(race, culture);
        let family = FamilyStructure::random(culture);


        Self {
            illegitimacy_info
        }
    }
}