//! 759: Unusual Pets

use dicebag::DiceExt;
use serde::{Deserialize, Serialize};

/// A few animal types.
#[derive(Debug, Deserialize, Serialize, Clone, Hash)]
pub enum Animal {
    /// A baby bear which *stays* a baby bear *indefinitely*.
    BabyBear,
    /// Some "big cat" (lion, tiger, etc.).
    BigCat,
    Bunny,
    Cat, Kitten,
    Dog, Puppy,
    Ferret,
    Fish,
    /// A fish that survives out of water indefinitely.
    FishOutOfWater,
    Hawk,
    Lizard,
    /// Mini-dragon might be an actual dragon of sorts, or just something that resembles a "dragon" to some degree.
    MiniDragon,
    Monkey,
    Mouse,
    Raccoon,
    Rat,
    /// Some rodent of player's choice.
    RodentOfChoice,
    Snake,
    /// Now… it's something… utterly alien. Let imagination on loose!
    SomethingAlien,
    Songbird,
}

impl Animal {
    /// Generate a random animal.
    pub fn new() -> Self {
        match 1.d20() {
            ..=2 => Self::Dog,
            3 => Self::Cat,
            4 => if 1.d2() == 1 {Self::Cat} else {Self::Kitten},
            5 => Self::Bunny,
            6 => Self::Lizard,
            7 => Self::Monkey,
            8 => Self::Raccoon,
            9 => if 1.d2() == 1 {Self::Rat} else {Self::Mouse},
            10 => Self::Snake,
            11 => Self::Hawk,
            12 => Self::RodentOfChoice,
            13 => Self::Ferret,
            14 => Self::Songbird,
            15 => if 1.d6() <= 6 {Self::FishOutOfWater} else {Self::Fish},
            16 => Self::Puppy,
            17 => Self::MiniDragon,
            18 => Self::BigCat,
            19 => Self::BabyBear,
            _ => Self::SomethingAlien
        }
    }
}
