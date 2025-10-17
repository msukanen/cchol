//! 102: Culture
use dicebag::DiceExt;
use rpgassist::modifier::HasModifier;
use serde::{Deserialize, Serialize};

use crate::society::environment::NativeEnvironment;

/// Culture level types for internal matcharoo.
#[derive(Debug)]
pub(crate) enum CultureLevelType {
    Primitive, Nomad, Barbarian, Civilized, Decadent
}

/// Various culture levels.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
pub enum Level {
    Primitive,
    Nomad,
    Barbarian(NativeEnvironment),
    Civilized(NativeEnvironment),
    Decadent
}

impl HasModifier for Level {
    /// CuMod
    fn modifier(&self) -> i32 {
        match self {
            Self::Primitive => -3,
            Self::Nomad => 0,
            Self::Barbarian(_) => 2,
            Self::Civilized(_) => 4,
            Self::Decadent => 7
        }
    }
}

impl Level {
    /// Generate random culture level.
    pub fn new() -> Self {
        match 1.d10() {
            ..=1 => Self::Primitive,
            2|3 => Self::Nomad,
            4..=6 => Self::Barbarian(NativeEnvironment::new(Some(NativeEnvironment::Wilderness))),
            7..=9 => Self::Civilized(NativeEnvironment::new(Some(NativeEnvironment::Urban))),
            _ => Self::Decadent
        }
    }

    /// Get culture's [native environment][NativeEnvironment].
    pub fn native_env(&self) -> NativeEnvironment {
        match self {
            Self::Barbarian(e) |
            Self::Civilized(e) => *e,
            Self::Decadent => NativeEnvironment::Urban,
            _ => NativeEnvironment::Wilderness
        }
    }
}
