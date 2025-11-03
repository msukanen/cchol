use std::fs;

use dicebag::DiceExt;
use lazy_static::lazy_static;
use rpgassist::ranking::{Rank, rank::IsRanked};
use serde::{Deserialize, Serialize};

use crate::{IsNamed, traits::personality::{self, AffectsAlignment, Alignment}};

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum OccupationAchievementLevel {
    Apprentice { skill_rank: u8 },
    Journeyman { skill_rank: u8 },
    SkilledTradesman { skill_rank: u8 },
    MasterCraftsman { skill_rank: u8 },
    MasterOfNote { skill_rank: u8 }
}

impl IsRanked for OccupationAchievementLevel {
    fn rank(&self) -> Rank {
        match self {
            Self::Apprentice { skill_rank }     |
            Self::Journeyman { skill_rank }     |
            Self::MasterCraftsman { skill_rank }|
            Self::MasterOfNote { skill_rank }   |
            Self::SkilledTradesman { skill_rank } => Rank::from(skill_rank)
        }
    }
}

impl OccupationAchievementLevel {
    pub fn random() -> Self {
        match 1.d20() {
            ..=2 => Self::Apprentice { skill_rank: 1.d2() },
            ..=14 => Self::Journeyman { skill_rank: 1.d3() + 2 },
            ..=17 => Self::SkilledTradesman { skill_rank: 1.d2() + 4 },
            ..=19 => Self::MasterCraftsman { skill_rank: 1.d2() + 6 },
            _ => Self::MasterOfNote { skill_rank: match 1.d20() {
                ..=17 => 9,
                ..=19 => 10,
                _ => 11
            } }
        }
    }
}

static WORK_ATTITUDES_FILE: &'static str = "./data/work_attitude.json";
lazy_static! {
    static ref WORK_ATTITUDES: Vec<WorkAttitude> = {
        serde_jsonc::from_str(
            &fs::read_to_string(WORK_ATTITUDES_FILE).expect(format!("Error with '{WORK_ATTITUDES_FILE}'?!").as_str())
        ).expect("JSON error")
    };
    static ref WORK_ATTITUDES_COUNT: usize = WORK_ATTITUDES.len();
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct WorkAttitude {
    l: String,
    d: String,
    #[serde(default)]
    alignment: Alignment,
}

impl WorkAttitude {
    /// Pick random work attitude.
    fn random() -> Self {
        let roll = 1.d(*WORK_ATTITUDES_COUNT + 1);
        let lightside = 1.d3() > 1;
        if roll > *WORK_ATTITUDES_COUNT {
            let t = (if lightside {
                personality::random_lightside(&vec![])
            } else {
                personality::random_darkside(&vec![])
            }).as_vec()[0].clone();
            
            if lightside {
                Self { l: t.name(), d: String::new(), alignment: Alignment::L }
            } else {
                Self { l: String::new(), d: t.name(), alignment: Alignment::D }
            }
        } else {
            let a = &(*WORK_ATTITUDES)[roll - 1];
            Self { alignment: if lightside {Alignment::L} else {Alignment::D}, .. a.clone() }
        }
    }
}

impl IsNamed for WorkAttitude {
    fn name(&self) -> String {
        match self.alignment {
            Alignment::D => self.d.clone(),
            _ => self.l.clone()
        }
    }
}

impl AffectsAlignment for WorkAttitude {
    fn alignment(&self) -> Alignment {
        self.alignment
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct  OccupationPerformance {
    attitude: WorkAttitude,
    oa_lvl: OccupationAchievementLevel,
}

impl IsRanked for OccupationPerformance {
    fn rank(&self) -> Rank {
        self.oa_lvl.rank()
    }
}

impl OccupationPerformance {
    fn random() -> Self {
        Self { attitude: WorkAttitude::random(), oa_lvl: OccupationAchievementLevel::random() }
    }
}

#[cfg(test)]
mod work_attitude_tests {
    use crate::misc::occupation::WORK_ATTITUDES_COUNT;

    #[test]
    fn work_attitude_data_integrity() {
        assert!(19 <= *WORK_ATTITUDES_COUNT);
    }
}