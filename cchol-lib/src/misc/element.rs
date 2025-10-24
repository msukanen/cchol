use std::fmt::Display;

use dicebag::DiceExt;
use serde::{Deserialize, Serialize};

/// Various "elements".
#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
pub enum Element {
    Air,
    Earth,
    Fire,
    /// Spirit isn't a physical phenomena like the others, but…
    Spirit,
    Water,
}

impl Display for Element {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Air => write!(f, "air"),
            Self::Earth => write!(f, "earth"),
            Self::Fire => write!(f, "fire"),
            Self::Spirit => write!(f, "spirit"),
            Self::Water => write!(f, "water"),
        }
    }
}

impl Element {
    /// Generate a random "element".
    pub fn new() -> Self {
        match 1.d10() {
            ..=2 => Self::Air,
            ..=5 => Self::Earth, // Earth - the most prominent of all.
            ..=7 => Self::Fire,
            8 => Self::Spirit,   // Spirit - a bit rare these.
            _ => Self::Water
        }
    }
}