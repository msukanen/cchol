//! # Native Environments
//! 
//! Wilderness is a very vague term, applicable for almost anything non-Urban…
//! 
use serde::{de, Deserialize, Serialize};

/// Some native environments.
#[derive(Debug, Serialize, PartialEq, Eq, Clone)]
pub enum NativeOf {
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
                _ => Err(de::Error::unknown_variant(&s, &["Urban", "Wilderness"]))
            },
            Halp::C(c) => Ok(NativeOf::Choice { primary: c.primary, secondary: c.secondary })
        }
    }
}

impl NativeOf {
    /// Get primary [NativeOf].
    pub fn primary(&self) -> NativeOf {
        match self {
            Self::Choice { primary,..} => *primary.clone(),
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
            Self::Urban => Self::Wilderness,
            Self::Wilderness => Self::Urban,
            Self::Choice { primary, .. } => primary.opposite()
        }
    }
}