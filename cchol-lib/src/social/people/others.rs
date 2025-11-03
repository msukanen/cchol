//! 750: Others
use rpgassist::gender::{Gender, HasGender};
use serde::{Deserialize, Serialize};

use crate::{racial::Monster, social::{nobility::SimpleNobleNPC, people::govt_official::GovtOfficial}};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum OutcastType {
    Beggar,
    Hermit,
    Leper,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum WOMType {
    AmazingAlchemist,
    DaringDruid,
    PowerfulPriest,
    WondrousWizard,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum OtherPeople {
    Combined(Vec<Box<OtherPeople>>),
    Friend { gender: Gender },
    GovtOfficial(GovtOfficial),
    Mentor { gender: Gender },
    Monster(Monster),
    Neighbor { gender: Gender },
    Noble { specs: SimpleNobleNPC },
    Outcast { r#type: OutcastType, gender: Gender },
    Prostitute,
    Thief { gender: Gender },
    WielderOfMagic { r#type: WOMType, gender: Gender },
}

impl OtherPeople {
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
            Self::Friend { gender }    |
            Self::Mentor { gender }    |
            Self::Neighbor { gender }  |
            Self::Outcast { gender,..} |
            Self::Thief { gender }     |
            Self::WielderOfMagic { gender,.. } => *gender,
            _ => unimplemented!()
        }
    }
}