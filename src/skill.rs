use rpgassist::ranking::rank::IsRanked;

use crate::named::IsNamed;

pub mod literacy;
pub mod survival;

/// A core trait for anything skill/skill-like.
pub trait IsSkill : IsRanked + IsNamed {}
