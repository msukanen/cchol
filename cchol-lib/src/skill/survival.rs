use std::ops::{Add, AddAssign, Sub, SubAssign};

use rpgassist::ranking::{rank::IsRanked, Rank};

use crate::{named::IsNamed, skill::IsSkill};

pub struct SurvivalSkill {
    rank: Rank,
    name: String,
}

impl IsSkill for SurvivalSkill {}
impl IsNamed for SurvivalSkill {
    fn name(&self) -> &str {
        &self.name
    }
}

impl IsRanked for SurvivalSkill {
    fn rank(&self) -> Rank {
        self.rank
    }
}

impl Add<i32> for SurvivalSkill {
    type Output = Self;
    fn add(self, rhs: i32) -> Self::Output {
        Self { rank: self.rank + rhs, .. self}
    }
}

impl Sub<i32> for SurvivalSkill {
    type Output = Self;
    fn sub(self, rhs: i32) -> Self::Output {
        Self { rank: self.rank - rhs, .. self}
    }
}

impl AddAssign<i32> for SurvivalSkill {
    fn add_assign(&mut self, rhs: i32) {
        self.rank += rhs
    }
}

impl SubAssign<i32> for SurvivalSkill {
    fn sub_assign(&mut self, rhs: i32) {
        self.rank -= rhs
    }
}
