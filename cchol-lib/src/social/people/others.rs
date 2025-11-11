//! 750: Others
use std::fs;

use dicebag::DiceExt;
use lazy_static::lazy_static;
use rpgassist::{gender::{Gender, GenderBias, HasGender}, ext::IsNamed};
use serde::{Deserialize, Serialize};

use crate::{misc::OccupationPerformance, racial::{Monster, Race}, social::{nobility::SimpleNobleNPC, people::{Relation, Rival, adventurer::Adventurer, govt_official::{self, GovtOfficial}}}, traits::HasCulture};

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
    Adventurer(Adventurer),
    CommonSoldier { gender: Gender },
    /// A variety of [CRIMINAL_TYPES].
    Criminal { r#type: String, gender: Gender,
        // the degree of involvement on 1…20 scale, from brief brush to being The Kingpin of the "trade".
        deg_of_involvement: u8
    },
    Friend { gender: Gender },
    GovtOfficial(GovtOfficial),
    Invader,
    KnownByOccupation { occupation: OccupationPerformance, gender: Gender },
    Lover { gender: Gender },
    Mentor { gender: Gender },
    Monster(Monster),
    Neighbor { gender: Gender },
    Noble { specs: SimpleNobleNPC },
    Nonhuman {
        // for non-human we need just name of the race here, any other specs are irrelevant in this context.
        race: String, gender: Gender },
    Outcast { r#type: OutcastType, gender: Gender },
    Prostitute,
    Relative { relation: Relation },
    Rival { specs: Rival },
    Thief { gender: Gender },
    WielderOfMagic { r#type: WOMType, gender: Gender },
    WildAnimal { gender: Gender },//TODO: some common(ish) animal types.
} impl OtherPeople {
    /// Generate some random peep(s).
    pub fn random(culture: &impl HasCulture) -> Self {
        match 1.d20() {
            ..=1 => Self::GovtOfficial(govt_official::random()),
            2 => Self::Friend { gender: Gender::random() },
            3 => if 1.d3() == 1 {Self::Prostitute} else {Self::Outcast { r#type: OutcastType::random(), gender: Gender::random() }},
            4 => Self::WielderOfMagic { r#type: WOMType::random(), gender: Gender::random() },
            5 => Self::Mentor { gender: Gender::random() },
            6 => Self::Thief { gender: Gender::random() },
            7 => Self::Noble { specs: SimpleNobleNPC::new("<name>") },
            8 => Self::Monster(Monster::random()),
            9 => Self::Neighbor { gender: Gender::random() },
            10 => Self::Lover { gender: Gender::random_biased(GenderBias::Female23) },
            11 => Self::KnownByOccupation { occupation: unimplemented!("TODO 420,421,422,423"), gender: Gender::random() },
            12 => Self::WildAnimal { gender: Gender::random() },
            13 => Self::Invader,
            14 => Self::CommonSoldier { gender: Gender::random_biased(GenderBias::Male23) },
            15 => Self::Criminal {
                r#type: CRIMINAL_TYPES[1.d(CRIMINAL_TYPES.len())-1].clone(),
                gender: Gender::random_biased(GenderBias::Male23),
                deg_of_involvement: 1.d20() },
            16 => Self::Adventurer(Adventurer::random()),
            17 => Self::Relative { relation: Relation::random() },
            18 => Self::Rival { specs: Rival::random(culture) },
            19 => Self::Nonhuman { race: Race::random_nonhuman().name().into(), gender: Gender::random() },
            _ => {// lets make a 2-3 combo variant
                let mut cmb = vec![Box::new(Self::random(culture))];
                for _ in 0..1.d2() {
                    cmb.push(Box::new(Self::random(culture)));
                }
                Self::Combined(cmb)
            }
        }
    }
}

impl HasGender for OtherPeople {
    fn gender(&self) -> Gender {
        match self {
            // Combined is… combined. Find the gender(s) via other means.
            Self::Combined(_) => Gender::Unspecified,

            Self::Adventurer(a)       => a.gender(),
            Self::GovtOfficial(g)   => g.gender(),
            Self::Noble { specs } => specs.gender(),
            Self::Monster(m)             => m.gender(),
            Self::Relative { relation } => relation.gender(),
            Self::Rival { specs }          => specs.gender(),
            //--- ones with a direct 'gender' field:
            Self::CommonSoldier { gender }       |
            Self::Criminal { gender,.. }         |
            Self::Friend { gender }              |
            Self::KnownByOccupation { gender,.. }|
            Self::Lover { gender }               |
            Self::Mentor { gender }              |
            Self::Neighbor { gender }            |
            Self::Nonhuman { gender,..}          |
            Self::Outcast { gender,..}           |
            Self::Thief { gender }               |
            Self::WielderOfMagic { gender,.. }   |
            Self::WildAnimal { gender }          => *gender,
            //--- the ones with fixed gender…:
            Self::Prostitute => Gender::Female,
            Self::Invader    => Gender::Male,
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