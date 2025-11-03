//! 750: Others
use std::fs;

use dicebag::DiceExt;
use lazy_static::lazy_static;
use rpgassist::gender::{Gender, HasGender};
use serde::{Deserialize, Serialize};

use crate::{misc::OccupationPerformance, racial::Monster, social::{nobility::SimpleNobleNPC, people::govt_official::GovtOfficial}};

static CRIMINAL_TYPES_FILE: &'static str = "./data/criminals.json";
lazy_static! {
    static ref CRIMINAL_TYPES: Vec<String> = serde_jsonc::from_str(
        &fs::read_to_string(CRIMINAL_TYPES_FILE).expect(format!("Error with '{CRIMINAL_TYPES_FILE}'?!").as_str())
    ).expect("JSON error");
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum OutcastType {
    Beggar,
    Hermit,
    Leper,
} impl OutcastType {
    fn random() -> Self {
        match 1.d10() {
            ..=6 => Self::Beggar,
            ..=9 => Self::Hermit,
            _ => Self::Leper
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum WOMType {
    AmazingAlchemist,
    DaringDruid,
    PowerfulPriest,
    WondrousWizard,
} impl WOMType {
    fn random() -> Self {
        match 1.d4() {
            ..=1 => Self::AmazingAlchemist,
            2 => Self::DaringDruid,
            3 => Self::PowerfulPriest,
            _ => Self::WondrousWizard
        }
    }
}

/// Types of "other people" (which are not always necessarily people, but still…).
#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum OtherPeople {
    Combined(Vec<Box<OtherPeople>>),
    CommonSoldier { gender: Gender },
    /// A variety of [CRIMINAL_TYPES].
    Criminal { r#type: String, gender: Gender,
        // the degree of involvement on 1…20 scale, from brief brush to being The Kingpin of the "trade".
        deg_of_involvement: u8
    },
    Friend { gender: Gender },
    GovtOfficial(GovtOfficial),
    Invader { gender: Gender },
    KnownByOccupation { occupation: OccupationPerformance, gender: Gender },
    Lover { gender: Gender },
    Mentor { gender: Gender },
    Monster(Monster),
    Neighbor { gender: Gender },
    Noble { specs: SimpleNobleNPC },
    Outcast { r#type: OutcastType, gender: Gender },
    Prostitute,
    Thief { gender: Gender },
    WielderOfMagic { r#type: WOMType, gender: Gender },
    //TODO: some common(ish) animal types.
    WildAnimal { gender: Gender },
} impl OtherPeople {
    pub fn random() -> Self {
        unimplemented!()
    }
}

impl HasGender for OtherPeople {
    fn gender(&self) -> Gender {
        match self {
            // Combined is… combined. Find the gender(s) via other means.
            Self::Combined(_) => Gender::Unspecified,
            Self::GovtOfficial(g) => g.gender(),
            Self::Prostitute => Gender::Female,
            Self::Noble { specs } => specs.gender(),
            Self::Monster(m) => m.gender(),
            //--- ones with a direct 'gender' field:
            Self::CommonSoldier { gender }       |
            Self::Criminal { gender,.. }         |
            Self::Friend { gender }              |
            Self::Invader { gender }             |
            Self::KnownByOccupation { gender,.. }|
            Self::Lover { gender }               |
            Self::Mentor { gender }              |
            Self::Neighbor { gender }            |
            Self::Outcast { gender,..}           |
            Self::Thief { gender }               |
            Self::WielderOfMagic { gender,.. }   |
            Self::WildAnimal { gender }          => *gender,
            //_ => unimplemented!()
        }
    }
}

#[cfg(test)]
mod other_people_tests {
    use crate::social::people::others::CRIMINAL_TYPES;

    #[test]
    fn criminals_file_data_integrity() {
        assert!(20 <= CRIMINAL_TYPES.len());
    }
}