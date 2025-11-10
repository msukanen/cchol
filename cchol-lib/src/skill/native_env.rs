//! # Native Environments
//! 
//! Wilderness is a very vague term, applicable for almost anything non-Urban…
//! 
use std::fmt::Display;

use rpgassist::ranking::rank::IsRanked;
use serde::{de, Deserialize, Serialize};

use crate::skill::{Skill, SkillBase};

pub static NATIVE_ENV_URBAN_SKILL_NAME:&'static str = "Survival: Urban";
pub static NATIVE_ENV_WILDS_SKILL_NAME:&'static str = "Survival: Wilderness";
pub static NATIVE_ENV_SHIP_SAILING_SKILL_NAME:&'static str = "Sailing: Ship";

/// Some native environments.
#[derive(Debug, Serialize, PartialEq, Eq, Clone)]
pub enum NativeOf {
    Air, Underground, Aquatic,
    WaterStructure { specific: String },
    Urban,
    Wilderness,
    /// Some cultures exist in any/all environments…
    Choice {
        /// The culture is primarily found in this environment…
        primary: Box<NativeOf>,
        /// …and occasionally here too.
        secondary: Box<NativeOf>,
    },
}

impl Display for NativeOf {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Air => write!(f, "air"),
            Self::Underground => write!(f, "underground"),
            Self::Aquatic => write!(f, "aquatic"),
            Self::WaterStructure { specific } => write!(f, "{}", specific),
            Self::Urban => write!(f, "urban"),
            Self::Wilderness => write!(f, "wilderness"),
            Self::Choice {
                primary,
                secondary
            } => write!(f, "{}|{}", primary, secondary)
        }
    }
}

/// A trait for anything where [NativeOf] is of any concern.
pub trait IsNativeOf {
    /// Get the [NativeOf].
    fn native_of(&self) -> NativeOf;
}

impl<'de> Deserialize<'de> for NativeOf {
    /// Custom deserializer for [NativeOf] fields.
    /// 
    /// # JSON
    /// ```json
    /// "native_of": "wilderness"
    /// "native_of": { "primary": "wilderness", "secondary": "urban" }
    /// ```
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where D: serde::Deserializer<'de>
    {
        #[derive(Deserialize)]
        struct ChoiceHalp {
            primary: Box<NativeOf>,
            secondary: Box<NativeOf>
        }
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum Halp {
            S(String),
            C(ChoiceHalp),
        }

        match Halp::deserialize(deserializer)? {
            Halp::S(s) => match s.to_ascii_lowercase().as_str() {
                "urban" => Ok(NativeOf::Urban),
                "wilderness" => Ok(NativeOf::Wilderness),
                // specials
                "air" => Ok(NativeOf::Air),
                "underground" => Ok(NativeOf::Underground),
                "aquatic" => Ok(NativeOf::Aquatic),
                // specific Water-things…
                "ship" => Ok(NativeOf::WaterStructure { specific: s }),
                // Uh-oh …
                _ => Err(de::Error::unknown_variant(&s, &["Urban", "Wilderness"]))
            },
            Halp::C(c) => Ok(NativeOf::Choice { primary: c.primary, secondary: c.secondary })
        }
    }
}

impl From<&str> for NativeOf {
    fn from(value: &str) -> Self {
        match value.to_lowercase().as_str() {
            "air" => Self::Air,
            "underground" => Self::Underground,
            "urban" => Self::Urban,
            "wilderness"|
            "wilds" => Self::Wilderness,
            "ship"|"raft"|"floating village"
                => Self::WaterStructure { specific: value.into() },
            // some synonymous aquatic entries…
            "lake"|"sea"|"ocean"|"river"|"pond"|
            "aquatic" => Self::Aquatic,
            _ => panic!("Environmental damage! '{value}' is not among the recognized ones of '{}', '{}', '{}' or '{}'!",
                NativeOf::Air, NativeOf::Underground, NativeOf::Urban, NativeOf::Wilderness
            )
        }
    }
}

impl NativeOf {
    /// Get primary [NativeOf].
    pub fn primary(&self) -> NativeOf {
        match self {
            Self::Choice { primary,..} => primary.primary(),
            _ => self.clone()
        }
    }

    /// Get secondary [NativeOf], if such is defined.
    pub fn secondary(&self) -> Option<NativeOf> {
        match self {
            Self::Choice { secondary,.. } => Some(*secondary.clone()),
            _ => None
        }
    }

    /// Get the "polar opposite" of primary [NativeOf].
    pub fn opposite(&self) -> NativeOf {
        match self {
            Self::Air => Self::Underground,
            Self::Underground => Self::Air,
            Self::Urban => Self::Wilderness,
            Self::Wilderness => Self::Urban,
            Self::WaterStructure { .. } => self.clone(),// doesn't have a direct opposite at all…
            Self::Aquatic => Self::Air,// could be anything land, but Air is fine…
            Self::Choice { primary, .. } => primary.opposite()
        }
    }

    /// Return a [Skill] representation of this [NativeOf], if possible.
    pub fn as_skill(&self, ranked: &impl IsRanked) -> Option<Skill> {
        // Skill name has to be properly set…
        let skill_name = match self {
            Self::Urban => NATIVE_ENV_URBAN_SKILL_NAME,
            Self::Wilderness => NATIVE_ENV_WILDS_SKILL_NAME,
            Self::Choice { primary,..} => return primary.as_skill(ranked),
            Self::WaterStructure { specific } => match specific.to_lowercase().as_str() {
                "ship" => NATIVE_ENV_SHIP_SAILING_SKILL_NAME,
                _ => return None
            }
            _ => return None
        };
        
        Some(Skill::from((
            SkillBase::from(skill_name),
            ranked.rank().clone()
        )))
    }

    /// Replace e.g. <NativeOf> with the actual environment name.
    pub fn skill_placeholder_replace(&self, source: &str) -> String {
        source
            .replace("<NativeOf>", self.primary().to_string().as_str())
            .replace("<NativeOf.opposite>", self.opposite().to_string().as_str())
    }
}