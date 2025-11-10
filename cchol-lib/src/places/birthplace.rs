//! 110: Place of Birth
//! 111: Exotic Birth Locations
//! 
//! # [`PlaceOfBirth`]
//! 
//! Encapsulates all there is to know about birthplace(s).
use dicebag::{DiceExt, lo, HiLo};
use serde::{Deserialize, Serialize};

use crate::{places::birthplace::exotic::ExoticPlaceOfBirth, racial::Race, social::{BiMod, CuMod, birth_legitimacy::LegitMod, culture::{Culture, CultureCoreType, HasCultureCoreType}}};
mod exotic;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum PlaceOfBirth {
    FamilyHome,
    AtHealer,   AtHospital, AtHealersGuildhall, // culture specific
    CarriageWhileTraveling,
    Cave, CommonBarn, // culture spec.
    ForeignLand(Box<PlaceOfBirth>),
    CaveNPrim, // cave, non-primitive
    MiddleOfField,
    Forest,
    Exotic(ExoticPlaceOfBirth),
}

impl BiMod for PlaceOfBirth {
    fn bimod(&self) -> i32 {
        match self {
            Self::AtHealer|
            Self::AtHealersGuildhall|
            Self::AtHospital => -7,
            Self::FamilyHome => -5,
            Self::CarriageWhileTraveling => 1,
            Self::CommonBarn |
            Self::Cave       => 1,
            Self::ForeignLand(p) => 2 + p.bimod(),
            Self::CaveNPrim  => 5,
            Self::MiddleOfField => 1,
            Self::Forest => 2,
            Self::Exotic(e) => e.bimod()
        }
    }
}

impl PlaceOfBirth {
    pub fn random(race: &'static Race, culture: &'static Culture, legit: &impl LegitMod) -> Self {
        match 1.d20() + legit.legitmod() + culture.cumod() {
            ..=6 => Self::FamilyHome,
            ..=9 => match culture.core_type() {
                CultureCoreType::Primitive |
                CultureCoreType::Nomad     => Self::AtHealer,
                CultureCoreType::Barbarian => if lo!() {Self::AtHealer} else {Self::AtHealersGuildhall},
                _ => if lo!() {Self::AtHealersGuildhall} else {Self::AtHospital}
            },
            10 => Self::CarriageWhileTraveling,
            11 => match culture.core_type() {
                CultureCoreType::Primitive |
                CultureCoreType::Nomad     => Self::Cave,
                _ => Self::CommonBarn
            },
            12|13 => Self::ForeignLand(Box::new(Self::random(race, culture, legit))),
            14 => match culture.core_type() {
                CultureCoreType::Barbarian |
                CultureCoreType::Civilized |
                CultureCoreType::Decadent  => Self::CaveNPrim,
                _ => Self::Cave
            },
            15 => Self::MiddleOfField,
            16 => Self::Forest,
            _  => Self::Exotic(ExoticPlaceOfBirth::random(race, culture, legit))
        }
    }
}