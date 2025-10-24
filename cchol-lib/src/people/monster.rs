//! 756: Monsters
//! 
//! "Monsters are people too!", and thus under `src/people`…

use dicebag::DiceExt;
use rpgassist::misc::color::ExoticColor;
use serde::{Deserialize, Serialize};

use crate::{misc::Element, racial::race::Race};

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

/// Various lycanthrope types.
#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum LycanType {
    Werebear,
    Wereboar,
    Wererat,
    Werewolf,
}

impl LycanType {
    /// Generate a random lycan.
    fn new() -> Self {
        match 1.d6() {
            ..=2 => Self::Werebear,
            3 => Self::Wereboar,
            4 => Self::Wererat,
            _ => Self::Werewolf
        }
    }
}

/// Various faerie types.
#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum FaerieType {
    Brownie,
    Pixie,
    Redcap,
    Sprite,
}

impl FaerieType {
    /// Generate a random faerie.
    fn new() -> Self {
        match 1.d8() {
            ..=1 => Self::Redcap,
            ..=3 => Self::Sprite,
            ..=5 => Self::Brownie,
            _ => Self::Pixie
        }
    }
}

/// Reptilian, batrachian, etc.
#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum ScalyType {
    Reptilian(Race),
    BatrachianHumanoid,
}

impl ScalyType {
    fn new() -> Self {
        match 1.d3() {
            ..=1 => Self::BatrachianHumanoid,
            _ => Self::Reptilian(Race::new_reptile())
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
    Undead(UndeadType),
    Beastman(Race),
    Lycanthrope(LycanType),
    Giant,
    DemonOrDevil,
    GiantAnimal,
    SeaMonster,
    Faerie(FaerieType),
    Scaly(ScalyType),
    Elemental(Element),
    /// Alien eldritch horrors, etc.
    HorrorFromBeyond,
    /// Dark elves, etc.
    EvilElf,
    /// Evil sort of dwarf.
    EvilDwarf,
    /// Living statue, golem, etc.
    Golem,
    GM756A,
    GM756B,
}

impl Monster {
    /// Generate a brand new random monster (or not-so-new in some cases…).
    pub fn new() -> Self {
        match 1.d20() {
            ..=1 => Self::EvilHumanoid,
            2 => Self::IckyBlob,
            3 => Self::Dragon(ExoticColor::new()),
            4 => Self::MythologicalBeast,
            5 => Self::Undead(UndeadType::new()),
            6 => Self::Beastman(Race::new_beastman()),
            7 => Self::Lycanthrope(LycanType::new()),
            8 => Self::Giant,
            9 => Self::DemonOrDevil,
            10 => Self::GiantAnimal,
            11 => Self::SeaMonster,
            12 => Self::Faerie(FaerieType::new()),
            13 => Self::Scaly(ScalyType::new()),
            14 => Self::Elemental(Element::new()),
            15 => Self::HorrorFromBeyond,
            16 => Self::EvilElf,
            17 => Self::EvilDwarf,
            18 => Self::Golem,
            19 => Self::GM756A,
            _  => Self::GM756B
        }
    }
}