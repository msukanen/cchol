use std::ops::{Add, AddAssign, Sub, SubAssign};

use rpgassist::ranking::{rank::IsRanked, Rank};

use crate::named::IsNamed;

pub mod literacy;
pub mod survival;

/// A core trait for anything skill/skill-like.
pub trait IsSkill : IsRanked + IsNamed + Add<i32> + Sub<i32> + AddAssign<i32> + SubAssign<i32> {}

pub struct GenericSkill {
    rank: Rank,
    name: String,
}

impl IsSkill for GenericSkill {}
impl IsNamed for GenericSkill {
    fn name(&self) -> &str {
        &self.name
    }
}
impl IsRanked for GenericSkill {
    fn rank(&self) -> Rank {
        self.rank
    }
}
impl Add<i32> for GenericSkill {
    type Output = Self;
    fn add(self, rhs: i32) -> Self::Output {
        Self { rank: self.rank + rhs, ..self}
    }
}
impl Sub<i32> for GenericSkill {
    type Output = Self;
    fn sub(self, rhs: i32) -> Self::Output {
        Self { rank: self.rank + rhs, ..self}
    }
}
impl SubAssign<i32> for GenericSkill {
    fn sub_assign(&mut self, rhs: i32) {
        self.rank -= rhs
    }
}
impl AddAssign<i32> for GenericSkill {
    fn add_assign(&mut self, rhs: i32) {
        self.rank += rhs
    }
}