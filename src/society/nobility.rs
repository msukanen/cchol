//! 758: Nobility

use dicebag::DiceExt;
use serde::{Deserialize, Serialize};

use rpgassist::gender::Gender;

use crate::society::culture::{CultureLevelType, Level};

/// A struct to haul around a barebones Noble NPC.
#[derive(Debug, Clone)]
pub struct NobleNPC {
    gender: Gender,
    culture: Level,
    //wealth: Wealth,
    title: Title,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, PartialOrd, Clone)]
pub enum Title {
    Hetman,
    Knight,
    Prince,
    Baronet,
    Baron,
    Count,
    Earl,
    Subchieftain,
    Jarl,
    Viscount,
    Chieftain,
    Marquis,
    Duke,
    Archduke,
    RoyalPrince,
    Kahn,
    King,
    HighKing,
    Emperor
}

impl Title {
    pub fn new(culture_type: &CultureLevelType) -> Self {
        match culture_type {
            CultureLevelType::Primitive =>
                match 1.d100() {
                    ..=1 => Self::HighKing,
                    ..=30 => Self::Chieftain,
                    _ => Self::Subchieftain
                },

            CultureLevelType::Nomad =>
                match 1.d100() {
                    ..=10 => Self::Kahn,
                    ..=40 => Self::Chieftain,
                    ..=80 => Self::Subchieftain,
                    _ => Self::Hetman
                },

            CultureLevelType::Barbarian =>
                match 1.d100() {
                    ..=2 => Self::HighKing,
                    ..=15 => Self::King,
                    ..=25 => Self::RoyalPrince,
                    ..=45 => Self::Chieftain,
                    ..=60 => Self::Jarl,
                    ..=70 => Self::Subchieftain,
                    ..=75 => Self::Baron,
                    ..=80 => Self::Prince,
                    _ => Self::Hetman
                },

            CultureLevelType::Civilized |
            CultureLevelType::Decadent =>
                match 1.d100() {
                    ..=1 => Self::Emperor,
                    ..=5 => Self::King,
                    ..=15 => Self::RoyalPrince,
                    ..=20 => Self::Archduke,
                    ..=25 => Self::Duke,
                    ..=35 => Self::Marquis,
                    ..=50 => Self::Viscount,
                    ..=60 => if 1.d2() == 1 {Self::Count} else {Self::Earl},
                    ..=75 => Self::Baron,
                    ..=78 => Self::Baronet,
                    ..=90 => Self::Prince,
                    _ => Self::Knight
                }
        }
    }
}