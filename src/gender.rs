use dicebag::DiceExt;
use serde::{Deserialize, Serialize};

/// Genders, obviously …
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub enum Gender {
    Unspecified,
    Female,
    Male
}

impl Gender {
    /// Generate a random gender, with or without bias toward one or the other.
    pub fn new(bias: Option<Gender>) -> Self {
        if 1.d20() +
        match bias {
            Some(Self::Male) => -2,
            Some(Self::Female) => 2,
            _ => 0
        } <= 10 { Self::Male }
        else { Self::Female }
    }
}