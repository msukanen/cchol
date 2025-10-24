//! 757: Adventurers

use dicebag::DiceExt;
use rpgassist::{gender::Gender, ranking::Rank};
use serde::{Deserialize, Serialize};

use crate::racial::race::Race;

/// Some adventurer "professions".
#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum AdventurerProfession {
    Wizard,
    Priest,
    Warrior,
    /// Thief, rogue, etc.
    Thief,
    Ranger,
    Druid,
    Shaman,
    Bard,
    MartialMonk,
}

/// Adventurer prowess. An amalgam of all what they're capable of.
#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum AdventurerProwess {
    Beginner(Rank),
    Veteran(Rank),
    LocalHero(Rank),
    GrandMasterOfProfession(Rank),
}

impl AdventurerProwess {
    /// Generate a random prowess level.
    fn new() -> Self {
        match 1.d20() {
            ..=5 => Self::Beginner(Rank::from(1.d4() - 1)),
            ..=14 => Self::Veteran(Rank::from(1.d2() + 3)),
            ..=19 => Self::LocalHero(Rank::from(1.d2() + 5)),
            _ => Self::GrandMasterOfProfession(Rank::from(1.d4() + 7)),
        }
    }
}

/// Adventurer!
/// 
/// Note that `race` is in most cases [Race::SameAsPC] unless dictated otherwise.
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Adventurer {
    gender: Gender,
    profession: AdventurerProfession,
    race: Race,
    prowess: AdventurerProwess,
}

impl Adventurer {
    /// Generate a brand new random [Adventurer].
    pub fn new() -> Self {
        let mut race = Race::SameAsPC;
        let mut roll_cap = 20;
        let profession = loop {
            match 1.d(roll_cap) {
                ..=2 => break AdventurerProfession::Wizard,
                ..=6 => break AdventurerProfession::Priest,
                ..=11 => break AdventurerProfession::Warrior,
                ..=13 => break AdventurerProfession::Thief,
                ..=15 => break AdventurerProfession::Ranger,
                16 => break AdventurerProfession::Druid,
                17 => break AdventurerProfession::Shaman,
                18 => break AdventurerProfession::Bard,
                19 => break AdventurerProfession::MartialMonk,
                _ => {
                    race = Race::new_nonhuman();
                    roll_cap = 19
                }
            }
        };

        Self {
            gender: Gender::new(None),
            profession,
            prowess: AdventurerProwess::new(),
            race
        }
    }
}