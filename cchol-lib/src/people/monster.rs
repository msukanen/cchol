//! 756: Monsters
//! 
//! "Monsters are people too!", and thus under `src/people`…

use dicebag::DiceExt;
use rpgassist::misc::color::ExoticColor;
use serde::{Deserialize, Serialize};

/// Some specific undead types.
#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum UndeadType {
    Ghost,
    Ghoul,
    Mummy,
    Skeleton,
    Spectre,
    Vampire,
    Zombie,
    Wight,
    Wraith,
}

impl UndeadType {
    /// Generate a random not-so-fresh undead.
    fn new() -> Self {
        match 1.d10() {
            ..=2 => Self::Ghost,
            3 => Self::Ghoul,
            4 => Self::Mummy,
            5 => Self::Skeleton,
            6 => Self::Spectre,
            7 => Self::Vampire,
            8 => Self::Wight,
            9 => Self::Wraith,
            _ => Self::Zombie
        }
    }
}

/// Various monster types.
#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum Monster {
    /// Orcs, trolls, goblins, etc.
    EvilHumanoid,
    /// Blobs, globs, slimes and jellies.
    IckyBlob,
    Dragon(ExoticColor),
    /// Anything from unicorn to sphinx and roc.
    MythologicalBeast,
    Undead,
}