//! 864: Deities

use dicebag::DiceExt;
use rpgassist::gender::{Gender, HasGender};
use serde::{Deserialize, Serialize};

use crate::modifier::CuMod;

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
    jaded: bool,
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

impl DeityType {
    fn maybe_rnd_gender(&self) -> Option<Gender> {
        match self {
            Self::AgriculturalGoddess => Some(Gender::Female),
            Self::AirGod => Some(Gender::new(Some(Gender::Male))),
            Self::AncestorWorship => None,
            Self::BeastGods => None,
            Self::EarthGoddess => Some(Gender::Female),
            Self::FireGod => Some(Gender::new(Some(Gender::Male))),
            Self::GodOfThieves => Some(Gender::new(None)),
            Self::GodOfWisdomAndKnowledge => Some(Gender::new(None)),
            Self::HealingGod => Some(Gender::new(Some(Gender::Female))),
            Self::HuntingGod => Some(Gender::Male),
            Self::LoveGoddess => Some(Gender::Female),
            Self::LuckGoddess => Some(Gender::Female),
            Self::MoonGoddess => Some(Gender::Female),
            Self::NightGoddess => Some(Gender::Female),
            Self::RulingDeity => Some(Gender::new(Some(Gender::Male))),
            Self::TradeGod => Some(Gender::new(Some(Gender::Male))),
            Self::Trickster => Some(Gender::new(None)),
            Self::UnderworldGod => Some(Gender::new(None)),
            Self::WarGod => Some(Gender::Male),
            Self::WaterGod => Some(Gender::new(Some(Gender::Male))),
        }
    }
}

impl Deity {
    /// Is the deity/religion decadent?
    pub fn is_decadent(&self) -> bool { self.jaded }
    pub fn core_type(&self) -> DeityType { self.kind }
    pub fn alignment(&self) -> DeityAlignment { self.alignment }
    pub fn new(cumod_src: &impl CuMod) -> Self {
        // ret: DT, jaded, evil, disguised
        let mut jaded = false;
        let mut evil = false;
        let mut disguised = false;
        let kind = loop {
            match 1.d20() + cumod_src.cumod() {
                ..=1 => break DeityType::AncestorWorship,
                2 => break DeityType::BeastGods,
                3 => break DeityType::HuntingGod,
                4 => break DeityType::Trickster,
                ..=6 => break DeityType::EarthGoddess,
                ..=8 => break DeityType::AgriculturalGoddess,
                ..=10 => break DeityType::RulingDeity,
                11 => break DeityType::WaterGod,
                12 => break DeityType::FireGod,
                13 => break DeityType::MoonGoddess,
                14 => break DeityType::AirGod,
                15 => {
                    if !disguised && evil {
                        disguised = true
                    }
                    evil = true
                },
                16 => break DeityType::WarGod,
                17 => break DeityType::LoveGoddess,
                18 => break DeityType::UnderworldGod,
                19 => break DeityType::GodOfWisdomAndKnowledge,
                20 => break DeityType::HealingGod,
                21 => break DeityType::TradeGod,
                22 => break DeityType::LuckGoddess,
                23 => break DeityType::NightGoddess,
                24 => break DeityType::GodOfThieves,
                _ => {
                    if jaded {
                        evil = true;
                        disguised = true;
                    }
                    jaded = true
                }
            }
        };

        Self {
            gender: kind.maybe_rnd_gender(),
            alignment:
                if evil {
                    DeityAlignment::Evil { disguised }
                } else {
                    DeityAlignment::Neutral
                },
            kind,
            jaded,
        }
    }
}
