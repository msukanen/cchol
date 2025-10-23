//! 864: Deities

use rpgassist::gender::{Gender, HasGender};
use serde::{Deserialize, Serialize};

/// The axis of evil for deities.
#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
pub enum DeityAlignment {
    Good,
    Neutral,
    Evil { disguised: bool },
}

/// Root religion/deity type.
#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
pub enum DeityType {
    AncestorWorship,
    BeastGods,
    HuntingGod,
    Trickster,
    EarthGoddess,
    AgriculturalGoddess,
    RulingDeity,
    /// Sea/water.
    WaterGod,
    /// Sun/fire.
    FireGod,
    MoonGoddess,
    /// Storm/air.
    AirGod,
    WarGod,
    LoveGoddess,
    UnderworldGod,
    GodOfWisdomAndKnowledge,
    HealingGod,
    TradeGod,
    LuckGoddess,
    NightGoddess,
    GodOfThieves,
}

/// Deity specs combined.
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Deity {
    alignment: DeityAlignment,
    kind: DeityType,
    decadent: bool,
    /// Some deities have a gender, but not all.
    gender: Option<Gender>,
}

impl HasGender for Deity {
    fn gender(&self) -> Gender {
        if let Some(gender) = &self.gender {
            gender.clone()
        } else { Gender::Unspecified }
    }
}

impl Deity {
    /// Is the deity/religion decadent?
    pub fn is_decadent(&self) -> bool { self.decadent }
    pub fn core_type(&self) -> DeityType { self.kind }
    pub fn alignment(&self) -> DeityAlignment { self.alignment }
}
