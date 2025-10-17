use dicebag::DiceExt;
use serde::{Deserialize, Serialize};

/// Native environments.
#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum NativeEnvironment {
    Wilderness,
    Urban,
}

impl NativeEnvironment {
    /// Generate a random native environment, with or without bias.
    pub fn new(bias_toward: Option<NativeEnvironment>) -> Self {
        match bias_toward {
            Some(e) => if 1.d3() == 1 {e.opposite()} else {e},
            _ => if 1.d2() == 1 {Self::Wilderness} else {Self::Urban}
        }
    }

    /// Get the "opposite" nat.env.
    fn opposite(&self) -> Self {
        match self {
            Self::Urban => Self::Wilderness,
            Self::Wilderness => Self::Urban
        }
    }
}
