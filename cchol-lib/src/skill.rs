use std::{fs, ops::{AddAssign, SubAssign}};

use lazy_static::lazy_static;
use rpgassist::{ranking::{rank::IsRanked, Rank}, ext::IsNamed};
use serde::{Deserialize, Serialize};

use crate::ext::HasDescription;

pub mod native_env;
mod unusual;
pub use unusual::generate_unusual_skills;
mod literacy;
pub use literacy::LitMod;
pub use literacy::IsLiteracySource;

pub trait IsSkill : IsRanked + IsNamed + HasDescription {}

static SKILL_FILE: &'static str = "./data/skill.json";
lazy_static! {
    pub(crate) static ref SKILLS: Vec<SkillBase> = {
        serde_jsonc::from_str(
            &fs::read_to_string(SKILL_FILE)
                .expect(format!("No '{}' found!", SKILL_FILE).as_str())
        ).expect("JSON validation error")
    };
}

#[derive(Debug, Deserialize, Clone)]
pub struct SkillBase {
    name: String,
    description: String,
}

impl From<&str> for SkillBase {
    /// Get a named skill from the static storage.
    /// Be mindful about typos, as we *intentionally* panic here if the skill is *not* found.
    fn from(value: &str) -> Self {
        SKILLS.iter()
            .find(|s| s.name == value)
            .expect(format!("No such skill as '{}' defined!", value).as_str())
            .clone()
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Skill {
    name: String,
    rank: Rank,
    description: String,
}

impl From<SkillBase> for Skill {
    /// Derive a [Rank-0][Rank] [Skill] from the given [SkillBase].
    fn from(value: SkillBase) -> Self {
        Self {
            rank: Rank::default(),
            name: value.name,
            description: value.description
        }
    }
}

impl From<&str> for Skill {
    /// Derive a [Rank-0][Rank] [Skill] from the given `value`, routing query via [SkillBase].
    fn from(value: &str) -> Self {
        Self::from(SkillBase::from(value))
    }
}

impl AddAssign<i32> for Skill {
    // just re-routing +=
    fn add_assign(&mut self, rhs: i32) {
        self.rank += rhs
    }
}

impl SubAssign<i32> for Skill {
    // just re-routing -=
    fn sub_assign(&mut self, rhs: i32) {
        self.rank -= rhs
    }
}

#[cfg(test)]
impl From<(String, Rank, String)> for Skill {
    fn from(value: (String, Rank, String)) -> Self {
        Self { name: value.0, rank: value.1, description: value.2 }
    }
}

impl From<(SkillBase, Rank)> for Skill {
    /// Derive a skill from the given [SkillBase] and assign some [Rank] for it.
    fn from(value: (SkillBase, Rank)) -> Self {
        Self { rank: value.1, ..Self::from(value.0) }
    }
}

impl IsSkill for Skill {}

impl IsNamed for Skill {
    fn name(&self) -> &str {
        &self.name
    }
}

impl IsRanked for Skill {
    fn rank(&self) -> Rank {
        self.rank.clone()
    }

    fn rank_mut(&mut self) -> &mut Rank {
        &mut self.rank
    }
}

impl HasDescription for Skill {
    fn description(&self) -> &str {
        &self.description
    }

    fn description_mut(&mut self) -> &mut String {
        &mut self.description
    }
}

#[cfg(test)]
mod skill_tests {
    use super::*;

    #[test]
    fn description_mut_works() {
        let mut s = Skill::from(("A skill".into(), Rank::from(3), "The skill's description!".into()));
        assert_eq!("The skill's description!", s.description());
        *s.description_mut() = "Joe's money making monkey skill".to_string();
        assert_eq!("Joe's money making monkey skill", s.description());
    }
}