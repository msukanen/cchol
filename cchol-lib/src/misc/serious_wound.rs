//! 870: Serious Wounds
use std::{collections::{HashMap, VecDeque}, fmt::Display};

use dicebag::{DiceExt, IsOne};
use rpgassist::{body::location::BodyLocation, direction::bilateral::Bilateral, stat::Stat};
use serde::{Deserialize, Deserializer, Serialize};

use crate::traits::{IsExplained, personality::{PersonalityTrait, TraitVec, exotic_trait, mental_affliction}};

fn deserialize_bdt_maff<'de, D>(deserializer: D) -> Result<Vec<PersonalityTrait>, D::Error>
where D: Deserializer<'de> {
    let pts: Vec<PersonalityTrait> = Vec::deserialize(deserializer)?;
    Ok(pts)
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum BrainDamageType {
    Combined(Vec<BrainDamageType>),
    StatAffected { stat: Stat },
    AllSkillsAffected { amount: i32 },
    #[serde(deserialize_with = "deserialize_bdt_maff")]
    MentalAffliction(Vec<PersonalityTrait>),
    OneSkillIncrAllOtherDecr { incr: i32, decr: i32 },
}

impl Display for BrainDamageType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AllSkillsAffected { amount } => write!(f, "all skills {amount:+}"),
            Self::Combined(dmg) => {
                write!(f, "{}", dmg.iter()
                    .map(|d| d.to_string())
                    .collect::<Vec<String>>()
                    .join(", "))
            },
            Self::MentalAffliction(affs) => {
                write!(f, "{}", affs.iter()
                    .map(|a| (*a).to_string())
                    .collect::<Vec<String>>()
                    .join(", "))
            },
            Self::OneSkillIncrAllOtherDecr { incr, decr } => write!(f, "one skill {incr:+} and all other skills {decr:+}"),
            Self::StatAffected { stat } => write!(f, "{stat}"),
        }
    }
}


impl BrainDamageType {
    /// Generate random brain damage(s).
    /// 
    // FYI: random() itself doesn't use `bans` for anything, it's just routed
    //      through to something else(s) which might have some use for it.
    //
    pub fn random(bans: &TraitVec) -> Self {
        #[derive(Debug, PartialEq, Eq, Hash)]
        enum BDT { B1, B2, B3, B4, B5, B6 }
        impl From<i32> for BDT { fn from(value: i32) -> Self {
                match value {
                    // the selector below uses 1d8, of which 7-8 are a reroll,
                    // and thus we handle just 6 entries here.
                    ..=1 => Self::B1,
                    2 => Self::B2,
                    3 => Self::B3,
                    4 => Self::B4,
                    5 => Self::B5,
                    _ => Self::B6
                }
            }
        }

        let mut bdts = HashMap::new();
        // Figure out who many BDT(s) to generate.
        let mut num = 1;
        while num > 0 {
            num -= 1;
            let r = 1.d8();
            if r > 6 {
                num += 1.d3() + 1;
                continue;
            }
            *bdts.entry(BDT::from(r)).or_insert(0) += 1;
        }

        // Transform each of the above collected BDT types+counts into
        // concrete [BrainDamageType] instances.
        let mut bdtv = VecDeque::new();
        bdts.iter().for_each(|(bdt, count)|{
            match bdt {
                BDT::B1 |
                BDT::B5 => {
                    let mut val = 0;
                    for _ in 0..*count {
                        val -= 1.d3()
                    }
                    bdtv.push_back(BrainDamageType::StatAffected { stat: match bdt {
                        BDT::B1 => Stat::Int { val },
                        _       => Stat::Dex { val }
                    }});
                },

                BDT::B2 => bdtv.push_back(BrainDamageType::AllSkillsAffected { amount: *count }),
                BDT::B3 => for _ in 0..*count {bdtv.push_back(BrainDamageType::MentalAffliction(mental_affliction::random(bans).as_vec()))},
                BDT::B4 => for _ in 0..*count {bdtv.push_back(BrainDamageType::MentalAffliction(exotic_trait::random(bans).as_vec()))},
                BDT::B6 => {
                    let mut incr = 0;
                    let mut decr = 0;
                    for _ in 0..*count {
                        incr += 1.d8();
                        decr -= 1.d6();
                    }
                    bdtv.push_back(BrainDamageType::OneSkillIncrAllOtherDecr { incr, decr });
                }
            }
        });

        // bdtv.len() can not be below 1. There'll always be at least that many entriest.
        match bdtv.len() {
            1 => bdtv.pop_front().unwrap(),
            _ => Self::Combined(bdtv.into())
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
pub enum SeriousWoundFootnote {
    Bdtfn7,
    Bdtfn8
}

impl Display for SeriousWoundFootnote {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Bdtfn7 => write!(f, "†"),
            Self::Bdtfn8 => write!(f, "††"),
        }
    }
}

impl IsExplained for SeriousWoundFootnote {
    fn explain(&self) -> String {
        match self {
            Self::Bdtfn7 => "without painkillers, the character must make an INT/Will check to perform any action requiring concentration",
            Self::Bdtfn8 => "movement speed is ¾ of normal",
        }.to_string()
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum SeriousWound {
    Combine2 { fst: Box<SeriousWound>, snd: Box<SeriousWound> },
    Combine3 { fst: Box<SeriousWound>, snd: Box<SeriousWound>, trd: Box<SeriousWound> },
    ImpressiveFacialScar(Stat),
    ImpressiveBodyScars(BodyLocation),
    EyePutOut(Bilateral),
    LoseSomeTeeth(u8),
    EarTornOut { which: Bilateral, deafened_side: bool },
    Disfigurement { app: Stat, cha: Stat },
    BrainDamage(BrainDamageType),
    InjuryCausesConstantPain { dex: Stat, str: Stat, footnote: SeriousWoundFootnote },
    KneeInjury { footnote1: SeriousWoundFootnote, footnote2: SeriousWoundFootnote },
    BodyPartSevered(BodyLocation),
    InjuryHealsBadly { dex: Stat, str: Stat },
    FootInjury { footnote: SeriousWoundFootnote },
    LungDamage { footnote1: SeriousWoundFootnote, con: Stat },
    StomachInjury { con: Stat },
    KidneyDamage { con: Stat },
    GenitalInjury,
    ThroatInjury { voice_loss_percentage: u8 },
    BackInjury { str: Stat },
    LiverDamage { con: Stat },
}

impl Display for SeriousWound {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::BackInjury { .. } => write!(f, "back injury"),
            Self::BodyPartSevered(bl) => write!(f, "body part ({bl}) severed"),
            Self::BrainDamage(bd) => write!(f, "brain damage ({bd})"),
            Self::Combine2 { fst, snd } => write!(f, "{} alongside {}", fst.to_string(), snd.to_string()),
            Self::Combine3 { fst, snd, trd } => write!(f, "{}, with {} and also {}", fst.to_string(), snd.to_string(), trd.to_string()),
            Self::Disfigurement { app, cha } => write!(f, "disfigured ({app} and {cha})"),
            Self::EarTornOut { which, deafened_side } => write!(f, "{which} ear torn out{}", if *deafened_side {" and that side completely deafened"} else {""}),
            Self::EyePutOut(lr) => write!(f, "{lr} eye put out"),
            Self::FootInjury { .. } => write!(f, "foot injury"),
            Self::GenitalInjury => write!(f, "genital injury"),
            Self::ImpressiveBodyScars(bl) => write!(f, "impressive scars at {bl}"),
            Self::ImpressiveFacialScar(_) => write!(f, "impressive factial scar"),
            Self::InjuryCausesConstantPain { .. } => write!(f, "<injury> causes constant pain"),
            Self::InjuryHealsBadly { .. } => write!(f, "<injury> healed badly"),
            Self::KidneyDamage { .. } => write!(f, "kidney damage"),
            Self::KneeInjury { .. } => write!(f, "knee injury"),
            Self::LiverDamage { .. } => write!(f, "liver damage"),
            Self::LoseSomeTeeth(n) => write!(f, "lost {}", if *n!=1 {format!("{} teeth", n)} else {"a tooth".to_string()}),
            Self::LungDamage { .. } => write!(f, "lung damage"),
            Self::StomachInjury { .. } => write!(f, "stomach injury"),
            Self::ThroatInjury { .. } => write!(f, "throat injury"),
        }
    }
}

impl SeriousWound {
    pub fn random(bans: &TraitVec) -> Self {
        match 1.d20() {
            ..=1 => Self::ImpressiveFacialScar(Stat::Dex { val: if 1.d2().is_one() {1} else {-1} }),
            2 => Self::ImpressiveBodyScars(BodyLocation::random()),
            3 => Self::EyePutOut(Bilateral::random_lr()),
            4 => Self::LoseSomeTeeth(1.d4() as u8),
            5 => Self::EarTornOut { which: Bilateral::random_lr(), deafened_side: 1.d10() > 6 },
            6 => Self::Disfigurement { app: Stat::App { val: -(1.d10()) }, cha: Stat::Cha { val: -(1.d10()) }},
            7 => Self::BrainDamage(BrainDamageType::random(bans)),
            8 => Self::InjuryCausesConstantPain { dex: Stat::Dex { val: -1 }, str: Stat::Str { val: -1 }, footnote: SeriousWoundFootnote::Bdtfn7 },
            9 => Self::KneeInjury { footnote1: SeriousWoundFootnote::Bdtfn7, footnote2: SeriousWoundFootnote::Bdtfn8 },
            10 => Self::BodyPartSevered({
                let side = Bilateral::random_lr();
                match 1.d6() {
                    ..=1 => BodyLocation::Hand(side),
                    2 => BodyLocation::Arm(side),
                    3 => BodyLocation::Foot(side),
                    4 => BodyLocation::Leg(side),
                    5 => BodyLocation::Thumb(side),
                    _ => BodyLocation::Fingers { count: 1.d3(), side }
                }}),
            11 => Self::InjuryHealsBadly { dex: Stat::Dex { val: -1 }, str: Stat::Str { val: -1 }},
            12 => Self::FootInjury { footnote: SeriousWoundFootnote::Bdtfn8 },
            13 => Self::LungDamage { footnote1: SeriousWoundFootnote::Bdtfn7, con: Stat::Con { val: -1 }},
            14 => Self::StomachInjury { con: Stat::Con { val: -1 }},
            15 => Self::KidneyDamage { con: Stat::Con { val: -1 }},
            16 => Self::GenitalInjury,
            17 => Self::ThroatInjury { voice_loss_percentage: 1.d10() * 10 },
            18 => Self::BackInjury { str: Stat::Str { val: -(1.d6()) }},
            19 => Self::LiverDamage { con: Stat::Con { val: -1 }},
            _ => {
                let fst = Box::new(Self::random(bans));
                let snd = Box::new(Self::random(bans));
                if 1.d2().is_one() {
                    Self::Combine2 { fst, snd }
                } else {
                    let trd = Box::new(Self::random(bans));
                    Self::Combine3 { fst, snd, trd }
                }
            }
        }
    }
}