use std::{collections::HashSet, hash::Hash};

use dicebag::{DiceExt, IsOne};
use rpgassist::{gender::Gender, stat::Stat};
use serde::{Deserialize, Serialize};

use crate::{body::Birthmark, StatMap, racial::Race, social::{BiMod, culture::Culture}, traits::personality::DLNTrait};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum Ubc3941 {
    E01 { stat: Stat },
    E0203,
    E0405,
    E06 { stat: Stat },
    E07,
    E0809,
    E10,
} impl Hash for Ubc3941 {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            Self::E01 { .. } => 1.hash(state),
            Self::E0203      => 2.hash(state),
            Self::E0405      => 3.hash(state),
            Self::E06 { .. } => 4.hash(state),
            Self::E07        => 5.hash(state),
            Self::E0809      => 6.hash(state),
            Self::E10        => 7.hash(state),
        }
    }
} impl PartialEq for Ubc3941 {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::E01 { .. }, Self::E01 { .. })|
            (Self::E0203, Self::E0203)          |
            (Self::E0405, Self::E0405)          |
            (Self::E06 { .. }, Self::E06 { .. })|
            (Self::E07, Self::E07)              |
            (Self::E0809, Self::E0809)          |
            (Self::E10, Self::E10)             => true,
            _ => false
        }
    }
} impl Eq for Ubc3941 {}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum Ubc4950 {
    E01, E02, E03
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum UnusualBirthCircumstance {
    UbcNe05,
    Ubc0610,
    Ubc1120,
    Ubc2123,
    Ubc2425,
    Ubc2627,
    Ubc2831 { separated_at_birth: bool, drastically_diff_persona: bool },
    Ubc3234,
    Ubc3537,
    Ubc38nn,
    Ubc3941(HashSet<Ubc3941>),
    Ubc4244(HashSet<Ubc3941>),
    Ubc4548 { curse: Curse },
    Ubc4950(Ubc4950),
    Ubc5153,
    Ubc5455,
    Ubc56nn,
    Ubc57nn,
    Ubc5862,
    Ubc6364 { tragedy: Tragedy },
    Ubc6569 { birthmark: Birthmark },
    Ubc7075 { curse: Curse },
    Ubc7681 { blessing: Blessing },
    Ubc8285 { gender: Gender },
    Ubc86nn,
    Ubc8788 { prophesy: DeathSituation },
    Ubc8993 { affliction: PhysicalAffliction },
    Ubc94nn { psi: PsionicAbility },
    Ubc9599 { gift: GiftOrLegacy },
    Ubc100 { ubc1: Box<UnusualBirthCircumstance>, ubc2: Box<UnusualBirthCircumstance> },
    Ubc101105,
    Ubc106110 {
        stats: StatMap,
        affliction: PhysicalAffliction,
        curse: Curse,
        dln: DLNTrait,
    },
    Ubc111xxx {
        stats: StatMap,
        affliction: PhysicalAffliction,
        gift: GiftOrLegacy,
        blessing: Blessing,
        deity: Deity,
    }
} impl UnusualBirthCircumstance {
    pub fn random(gender: &Gender, race: &Race, culture: &Culture, bimod_src: &impl BiMod) -> Self {
        let roll = 1.d100() + bimod_src.bimod();
        match roll {
            ..=5 => Self::UbcNe05,
            ..=10 => Self::Ubc0610,
            ..=20 => Self::Ubc1120,
            ..=23 => Self::Ubc2123,
            ..=25 => Self::Ubc2425,
            ..=27 => Self::Ubc2627,
            ..=31 => Self::Ubc2831 { separated_at_birth: 1.d5().is_one(), drastically_diff_persona: 1.d6().is_one() },
            ..=34 => Self::Ubc3234,
            ..=37 => Self::Ubc3537,
            38 => Self::Ubc38nn,
            n if roll <= 44 => {
                let mut ubc3941s = HashSet::new();
                let mut c = 1.d3();
                while c > 0 && ubc3941s.len() < 7 {
                    c -= 1;
                    let variant = match 1.d10() {
                        ..=1 => Ubc3941::E01 { stat: Stat::Mag { val: 1.d6() } },
                        ..=3 => Ubc3941::E0203,
                        ..=5 => Ubc3941::E0405,
                        6 => Ubc3941::E06 { stat: Stat::Mag { val: -1.d6() } },
                        7 => Ubc3941::E07,
                        ..9 => Ubc3941::E0809,
                        _ => Ubc3941::E10
                    };
                    if ubc3941s.contains(&variant) {
                        c += 1;
                        continue;
                    }
                    ubc3941s.insert(variant);
                }
                
                if n < 42 {Self::Ubc3941(ubc3941s)} else {Self::Ubc4244(ubc3941s)}
            },
            ..=48 => Self::Ubc4548 { curse: Curse::random(culture) },
            ..=50 => Self::Ubc4950( match 1.d10() {
                ..=6 => Ubc4950::E01,
                ..=9 => Ubc4950::E02,
                _ => Ubc4950::E03,
            }),
            ..=53 => Self::Ubc5153,
            ..=55 => Self::Ubc5455,
            56 => Self::Ubc56nn,
            57 => Self::Ubc57nn,
            ..=62 => Self::Ubc5862,
            ..=64 => Self::Ubc6364 { tragedy: Tragedy::random(culture) },
            ..=69 => Self::Ubc6569 { birthmark: Birthmark::random() },
            ..=75 => Self::Ubc7075 { curse: Curse::random(culture) },
            ..=81 => Self::Ubc7681 { blessing: Blessing::random(culture) },
            ..=85 => Self::Ubc8285 { gender: Gender::random() },
            86 => Self::Ubc86nn,
            ..=88 => Self::Ubc8788 { prophesy: DeathSituation::random() },
            ..=93 => Self::Ubc8993 { affliction: PhysicalAffliction::random(gender) },
            94 => Self::Ubc94nn { psi: PsionicAbility::random(race) },
            ..=99 => Self::Ubc9599 { gift: gifts_n_legacies::random(culture) },
            100 => {
                struct BogusBim {val: i32}
                let bimod_src = BogusBim { val: bimod_src.bimod() + 20 };
                impl BiMod for BogusBim { fn bimod(&self) -> i32 {self.val}}
                let ubc1 = Box::new(Self::random(gender, race, culture, &bimod_src));
                let ubc2 = Box::new(Self::random(gender, race, culture, &bimod_src));
                Self::Ubc100 { ubc1, ubc2 }
            },
            ..=105 => Ubc101105,
        }
    }
}