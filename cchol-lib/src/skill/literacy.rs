//! Literacy – it is a skill, but to keep things properly "generic"
//! in this library, it means that you either can or cannot read/write.

use std::ops::{Add, AddAssign, Sub, SubAssign};

use dicebag::DiceExt;
use rpgassist::ranking::{rank::IsRanked, Rank};

use crate::{modifier::{LitMod, LitModType}, named::IsNamed, skill::IsSkill};

pub trait MaybeLiterate : IsSkill {
    fn can_readwrite(&self) -> bool;
}

pub struct Language {
    can_rw: bool,
    name: String,
}

pub struct PotentialLanguage {
    litmod: LitModType,
    name: String,
}

impl LitMod for PotentialLanguage {
    fn litmod(&self) -> LitModType {
        self.litmod.clone()
    }
}

impl IsRanked for Language {
    /// Rank as such is irrelevant — the character either is or isn't literate in this particular language.
    fn rank(&self) -> Rank {
        if self.can_rw { Rank::AVERAGE } else { Rank::NONE }
    }
}

impl IsSkill for Language {}

impl IsNamed for Language {
    fn name(&self) -> &str {
        &self.name
    }
}

impl MaybeLiterate for Language {
    fn can_readwrite(&self) -> bool {
        self.can_rw
    }
}

impl Language {
    pub fn make_literate(&mut self) {
        self.can_rw = true
    }

    pub fn make_illiterate(&mut self) {
        self.can_rw = false
    }

    /// Set up a language.
    pub fn new(name: &str, literate_in: bool) -> Self {
        Self { can_rw: literate_in, name: name.to_string() }
    }
}

impl Add<i32> for Language {
    type Output = Self;
    /// Adding to [Language] is a no-op.
    fn add(self, rhs: i32) -> Self::Output {
        //TODO: more granular language command later.
        log::info!("add({rhs}) does nothing in this implementation of `Language`.");
        self
    }
}

impl Sub<i32> for Language {
    type Output = Self;
    /// Subtracting from [Language] is a no-op.
    fn sub(self, rhs: i32) -> Self::Output {
        //TODO: more granular language command later.
        log::info!("sub({rhs}) does nothing in this implementation of `Language`.");
        self
    }
}

impl AddAssign<i32> for Language {
    /// Adding to [Language] is a no-op.
    fn add_assign(&mut self, rhs: i32) {
        //TODO: more granular language command later.
        log::info!("add_assign({rhs}) does nothing in this implementation of `Language`.");
    }
}

impl SubAssign<i32> for Language {
    /// Subtracting from [Language] is a no-op.
    fn sub_assign(&mut self, rhs: i32) {
        //TODO: more granular language command later.
        log::info!("sub_assign({rhs}) does nothing in this implementation of `Language`.");
    }
}

pub trait HasLiteracyBenefit {
    fn literacy(&self) -> Vec<PotentialLanguage>;
}

impl PotentialLanguage {
    pub fn new(name: &str, litmod: LitModType) -> Self {
        Self { litmod, name: name.into() }
    }
}

impl From<PotentialLanguage> for Language {
    fn from(lang: PotentialLanguage) -> Self {
        Self {
            can_rw: 1.d100() <= lang.litmod(),
            name: lang.name.clone()
        }
    }
}

impl Add<i32> for PotentialLanguage {
    type Output = Self;
    fn add(self, rhs: i32) -> Self::Output {
        Self { litmod: self.litmod + rhs, ..self}
    }
}

impl Sub<i32> for PotentialLanguage {
    type Output = Self;
    fn sub(self, rhs: i32) -> Self::Output {
        Self { litmod: self.litmod - rhs, ..self}
    }
}

impl AddAssign<i32> for PotentialLanguage {
    fn add_assign(&mut self, rhs: i32) {
        self.litmod += rhs
    }
}

impl SubAssign<i32> for PotentialLanguage {
    fn sub_assign(&mut self, rhs: i32) {
        self.litmod -= rhs
    }
}

impl Add<LitModType> for PotentialLanguage {
    type Output = Self;
    fn add(self, rhs: LitModType) -> Self::Output {
        Self { litmod: self.litmod + rhs, ..self}
    }
}

impl Sub<LitModType> for PotentialLanguage {
    type Output = Self;
    fn sub(self, rhs: LitModType) -> Self::Output {
        Self { litmod: self.litmod - rhs, ..self}
    }
}
