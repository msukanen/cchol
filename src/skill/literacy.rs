//! Literacy – it is a skill, but to keep things properly "generic"
//! in this library, it means that you either can or cannot read/write.

use rpgassist::ranking::{rank::IsRanked, Rank};

use crate::{named::IsNamed, skill::IsSkill};

pub trait MaybeLiterate : IsSkill {
    fn can_readwrite(&self) -> bool;
}

pub struct Language {
    can_rw: bool,
    language_name: String,
}

impl IsRanked for Language {
    fn rank(&self) -> Rank {
        if self.can_rw { Rank::AVERAGE } else { Rank::NONE }
    }
}

impl IsSkill for Language {}

impl IsNamed for Language {
    fn name(&self) -> &str {
        &self.language_name
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
        Self { can_rw: literate_in, language_name: name.to_string() }
    }
}