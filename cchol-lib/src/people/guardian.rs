//! 754: Guardians

use dicebag::DiceExt;
use serde::{Deserialize, Serialize};

use crate::{misc::{benefit::{Benefit, BenefitType}, HasBenefits}, modifier::CuMod, people::{adventurers::Adventurer, monster::Monster, relatives::Relation}, racial::race::Race, society::{family::{generate_family, Family}, religion::Deity}};

/// Some guardians.
#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum Guardian {
    Relative(Relation),
    Orphanage,
    Family(Box<Family>),
    Temple(Deity),
    RaisedByNonhumans(Race),
    //SoldIntoSlavery(Slavery::new(...)) //TODO: T-539
    RaisedOnStreets { benefits: Vec<Benefit> },
    RaisedByThievesGuild,//TODO: T-534
    PassedBetweenRelatives,
    RaisedByAdventurer(Adventurer),
    GMOnly,//TODO: 978#754
    RaisedByBeastsInWild,
    RaisedByMonsters(Monster),
}

impl Guardian {
    pub fn new(cumod_src: &impl CuMod) -> Self {
        match 1.d20() {
            ..=5 => Self::Relative(Relation::new()),
            ..=8 => Self::Orphanage,
            ..=10 => Self::Family(Box::new(generate_family(cumod_src, true))),
            11 => Self::Temple(Deity::new(cumod_src)),
            12 => Self::RaisedByNonhumans(Race::new_nonhuman()),
            //13 => Self::SoldIntoSlavery(Slavery::new(cumod_src, ...?)),//TODO: T-539
            14 => Self::RaisedOnStreets { benefits: vec![Benefit::from((BenefitType::Skill, 1.d4(), "Urban Survival"))] },
            15 => Self::RaisedByThievesGuild,
            16 => Self::PassedBetweenRelatives,
            17 => Self::RaisedByAdventurer(Adventurer::new()),
            18 => Self::GMOnly,
            19 => Self::RaisedByBeastsInWild,
            _ => Self::RaisedByMonsters(Monster::new())
        }
    }
}

impl HasBenefits for Guardian {
    fn benefits(&self) -> Vec<Benefit> {
        match self {
            Self::RaisedOnStreets { benefits } => benefits.clone(),
            _ => vec![]
        }
    }
}