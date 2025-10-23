use dicebag::DiceExt;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
pub enum TimeOfYear {
    Spring,
    Summer,
    Autumn,
    Winter
}

impl TimeOfYear {
    /// Generate a random time of year.
    pub fn new() -> Self {
        match 1.d4() {
            ..=1 => Self::Autumn,
            2 => Self::Spring,
            3 => Self::Summer,
            _ => Self::Winter
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
#[serde(untagged)]
pub enum TimeOfBirth {
    Date { day: i32, month: i32 },
    Approximate(TimeOfYear)
}