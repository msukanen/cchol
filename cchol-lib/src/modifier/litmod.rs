use std::ops::{Add, AddAssign, Sub, SubAssign};

/// **LitMod** types.
#[derive(Debug, Clone)]
pub enum LitModType {
    /// Freely adds up with other modifiers.
    Additive(i32),
    /// When encountered, overrides all other modifiers.
    /// 
    /// If multiple FixedOverride are encountered at once, the one with *lowest* wins.
    FixedOverride(i32),
}

impl PartialEq for LitModType {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Self::Additive(a) => match other {
                Self::Additive(b) => a == b,
                Self::FixedOverride(_) => false
            },

            Self::FixedOverride(a) => match other {
                Self::FixedOverride(b) => a == b,
                Self::Additive(_) => false
            }
        }
    }
}

impl PartialOrd for LitModType {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self {
            Self::Additive(a) => match other {
                Self::Additive(b) => a.partial_cmp(b),
                // Additive always loses vs FixedOverride
                Self::FixedOverride(_) => Some(std::cmp::Ordering::Less)
            },

            Self::FixedOverride(a) => match other {
                // FixedOverride trumps Additive, always.
                Self::Additive(_) => Some(std::cmp::Ordering::Greater),
                // We compare reversed so that lower == "stronger".
                Self::FixedOverride(b) => b.partial_cmp(a),
            }
        }
    }
}

impl Add for LitModType {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        match self {
            Self::Additive(a) => match rhs {
                Self::Additive(b) => Self::Additive(a + b),
                _ => rhs
            },

            Self::FixedOverride(a) => match rhs {
                Self::FixedOverride(b) => Self::FixedOverride(if a > b {a} else {b}),
                _ => self
            }
        }
    }
}

impl Sub for LitModType {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        match self {
            Self::Additive(a) => match rhs {
                Self::Additive(b) => Self::Additive(a - b),
                _ => rhs
            },

            Self::FixedOverride(a) => match rhs {
                Self::FixedOverride(b) => Self::FixedOverride(if a < b {a} else {b}),
                _ => self
            }
        }
    }
}

impl Add<i32> for LitModType {
    type Output = Self;
    fn add(self, rhs: i32) -> Self::Output {
        match self {
            Self::Additive(a) => Self::Additive(a + rhs),
            Self::FixedOverride(a) => Self::FixedOverride(a + rhs)
        }
    }
}

impl AddAssign<i32> for LitModType {
    fn add_assign(&mut self, rhs: i32) {
        match self {
            Self::Additive(a) |
            Self::FixedOverride(a) => *a += rhs
        }
    }
}

impl Sub<i32> for LitModType {
    type Output = Self;
    fn sub(self, rhs: i32) -> Self::Output {
        match self {
            Self::Additive(a) => Self::Additive(a - rhs),
            Self::FixedOverride(a) => Self::FixedOverride(a - rhs)
        }
    }
}

impl SubAssign<i32> for LitModType {
    fn sub_assign(&mut self, rhs: i32) {
        match self {
            Self::Additive(a) |
            Self::FixedOverride(a) => *a -= rhs
        }
    }
}

impl PartialEq<i32> for LitModType {
    fn eq(&self, other: &i32) -> bool {
        match self {
            Self::Additive(a) |
            Self::FixedOverride(a) => a.eq(other)
        }
    }
}

impl PartialOrd<i32> for LitModType {
    fn partial_cmp(&self, other: &i32) -> Option<std::cmp::Ordering> {
        match self {
            Self::Additive(a) |
            Self::FixedOverride(a) => a.partial_cmp(other)
        }
    }
}

impl PartialOrd<LitModType> for i32 {
    fn partial_cmp(&self, other: &LitModType) -> Option<std::cmp::Ordering> {
        self.partial_cmp(&i32::from(other))
    }
}

impl PartialEq<LitModType> for i32 {
    fn eq(&self, other: &LitModType) -> bool {
        self.eq(&i32::from(other))
    }
}

impl From<&LitModType> for i32 {
    fn from(value: &LitModType) -> Self {
        match value {
            LitModType::Additive(a)|
            LitModType::FixedOverride(a) => *a
        }
    }
}

/// LitMod, a.k.a. "literacy modifier".
pub trait LitMod {
    fn litmod(&self) -> LitModType;
}

#[cfg(test)]
mod litmod_tests {
    use super::*;

    #[test]
    fn fixed_low_is_stronger() {
        let a = LitModType::FixedOverride(10);
        let b = LitModType::FixedOverride(5);
        assert!(a < b);
    }

    #[test]
    fn additive_loses_vs_fixed() {
        let a = LitModType::Additive(50);
        let b = LitModType::FixedOverride(5);
        assert!(a < b);
    }
}