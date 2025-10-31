//! 647: Lightside Traits
//! 648: Darkside Traits

use std::{collections::HashMap, fs};

use dicebag::DiceExt;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use crate::{IsNamed, ext::IsZero};

pub mod personality_al;
pub use personality_al::allergies;
pub mod personality_bt;
pub use personality_bt::behavior_tag;
pub mod personality_ex;
pub use personality_ex::exotic_trait;
pub mod personality_ma;
pub use personality_ma::mental_affliction;
pub mod personality_ph;
pub use personality_ph::phobias;
pub mod personality_sd;
pub use personality_sd::sexual_disorder;

pub(crate) type BanVec = Vec<PersonalityTrait>;

static PERSONALITY_FILE: &'static str = "./data/personality.json";
lazy_static! {
    static ref PERSONALITY_TRAITS: HashMap<String, PersonalityTrait> = {
        let traits_data: PersonalityTraits = serde_jsonc::from_str(
            &fs::read_to_string(PERSONALITY_FILE)
                .expect(format!("No '{}' found?!", PERSONALITY_FILE).as_str())
        ).expect("JSON error");

        let mut allmap = HashMap::new();

        for mut trait_ in traits_data.lightside_traits {
            trait_.classification = PersonalityTraitClassification::L;
            allmap.insert(trait_.name.clone(), trait_);
        }

        for mut trait_ in traits_data.darkside_traits {
            trait_.classification = PersonalityTraitClassification::D;
            allmap.insert(trait_.name.clone(), trait_);
        }

        for mut trait_ in traits_data.neutral_traits {
            trait_.classification = PersonalityTraitClassification::N;
            allmap.insert(trait_.name.clone(), trait_);
        }

        allmap
    };

    static ref LIGHTSIDE_TRAITS: Vec<&'static PersonalityTrait> = {
        PERSONALITY_TRAITS.values()
            .filter(|t| t.classification == PersonalityTraitClassification::L)
            .collect()
    };

    static ref DARKSIDE_TRAITS: Vec<&'static PersonalityTrait> = {
        PERSONALITY_TRAITS.values()
            .filter(|t| t.classification == PersonalityTraitClassification::D)
            .collect()
    };

    static ref NEUTRAL_TRAITS: Vec<&'static PersonalityTrait> = {
        PERSONALITY_TRAITS.values()
            .filter(|t| t.classification == PersonalityTraitClassification::N)
            .collect()
    };
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct PersonalityTraitExcluder {
    trait_type: String,
    name: String,
}

impl IsNamed for PersonalityTraitExcluder {
    fn name(&self) -> &str {
        &self.name
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
pub enum PersonalityTraitClassification {
    D,
    L,
    N,
    NotApplicable
}

impl Default for PersonalityTraitClassification {
    fn default() -> Self {
        Self::NotApplicable
    }
}

/// Various degrees/strength of personality traits.
#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
pub enum PersonalityTraitStrength {
    Trivial,
    Weak,
    Average,
    Strong,
    Driving,
    Obsessive
}

impl Default for PersonalityTraitStrength {
    fn default() -> Self {
        Self::Average
    }
}

impl PersonalityTraitStrength {
    /// Generate random personality trait strength.
    pub fn random() -> Self {
        match 1.d100() {
            ..=10 => Self::Trivial,
            ..=29 => Self::Weak,
            ..=59 => Self::Average,
            ..=79 => Self::Strong,
            ..=94 => Self::Driving,
            _     => Self::Obsessive
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct PersonalityTrait {
    name: String,
    #[serde(default)] mutually_excludes: Vec<PersonalityTraitExcluder>,
    #[serde(default)] classification: PersonalityTraitClassification,
    #[serde(default)] strength: PersonalityTraitStrength,
}

impl IsNamed for PersonalityTrait {
    fn name(&self) -> &str {
        &self.name
    }
}

#[derive(Debug, Deserialize)]
struct PersonalityTraits {
    lightside_traits: Vec<PersonalityTrait>,
    darkside_traits: Vec<PersonalityTrait>,
    neutral_traits: Vec<PersonalityTrait>,
}

pub enum TraitRollResult {
    Add(PersonalityTrait),
    AddMultiple(BanVec),
    Evolve { what: PersonalityTrait, to: PersonalityTrait },
    NoMatch
}

impl TraitRollResult {
    pub fn as_vec(&self) -> Vec<PersonalityTrait> {
        match self {
            Self::Add(t) => vec![(*t).clone()],
            Self::AddMultiple(v) => v
                .iter()
                .map(|v| PersonalityTrait { ..(*v).clone() })
                .collect::<Vec<PersonalityTrait>>(),
            Self::Evolve { to,.. } => vec![(*to).clone()],
            Self::NoMatch => vec![]
        }
    }
}

pub fn random_any(bans: &mut BanVec) -> TraitRollResult {
    match 1.d100() {
        ..=50 => TraitRollResult::NoMatch,
        ..=65 => random(bans, &PersonalityTraitClassification::N),
        ..=80 => random(bans, &PersonalityTraitClassification::L),
        ..=95 => random(bans, &PersonalityTraitClassification::D),
        _     => exotic_trait::random(bans)
    }
}

/// Generate a random personality trait which doesn't clash with any in the `bans` list.
/// 
/// Some combinations of potential clashes produce an "evolved" trait instead of a simple
/// addition to list…
/// 
/// ```
///   TraitRollResult::Evolve { what, to }
/// ```
/// 
/// …tells what to replace with what.
#[must_use = "The result tells what else needs to be done, if anything"]
pub fn random(
        bans: &BanVec,
        classification: &PersonalityTraitClassification
) -> TraitRollResult {
    let pool = match classification {
        PersonalityTraitClassification::D => &*DARKSIDE_TRAITS,
        PersonalityTraitClassification::L => &*LIGHTSIDE_TRAITS,
        PersonalityTraitClassification::N => &*NEUTRAL_TRAITS,
        _ => unimplemented!("TODO")
    };

    let mut bail_out_at_zero = 100;

    loop {
        bail_out_at_zero -= 1;
        
        let entry = pool[1.d(pool.len()) - 1].clone();
        let clash = bans.iter().find(|t|
            t.name == entry.name ||
            t.mutually_excludes.iter().any(|x| x.name() == entry.name()));
        match clash {
            None => return TraitRollResult::Add(entry),
            Some(clash_with) => {
                log::debug!("Clash: '{}' vs '{}'", clash_with.name(), entry.name());
                if  (clash_with.name() == "Pessimist" && entry.name() == "Optimist")||
                    (clash_with.name() == "Optimist" && entry.name() == "Pessimist") {
                        let evolved_trait = PERSONALITY_TRAITS.get("Pessimistic-Optimist")
                            .expect("DATA ERROR: 'Pessimistic-Optimist' not found!")
                            .clone();
                        return TraitRollResult::Evolve { what: clash_with.clone(), to: evolved_trait };
                    }
            }
        }

        if bail_out_at_zero.is_zero() {
            log::error!("All options exhausted!");
            return TraitRollResult::NoMatch;
            //panic!("All personality trait options exhausted! Call a medic (or a dev…)!");
        }
    }
}

impl TraitRollResult {
    /// Directly apply the trait roll result in place on the given trait vec.
    pub fn apply(self, upon: &mut BanVec) {
        match self {
            Self::Add(x) => upon.push(x),
            Self::AddMultiple(v) => upon.extend(v),
            Self::Evolve { what, to } =>
                *(upon.iter_mut().find(|x| x.name == what.name).unwrap()) = to,
            Self::NoMatch => ()
        }
    }
}

#[cfg(test)]
mod personality_tests {
    use super::*;

    #[test]
    fn bans() {
        let mut bans = vec![];
        let classification = &PersonalityTraitClassification::D;
        bans.push(PERSONALITY_TRAITS.get("Optimist").unwrap().clone());
        // lets try until we get TraitRollResult::Evolve
        let mut x = 0;
        let _ = env_logger::try_init();
        loop {
            x += 1;
            log::debug!("Rolling ... {x}");
            let t1 = random(bans.as_ref(), classification);
            match t1 {
                TraitRollResult::NoMatch|
                TraitRollResult::Add(_) => (),
                _ => {
                    t1.apply(&mut bans);
                    break;
                }
            }
        }
        assert_eq!(1, bans.len());
        assert_eq!("Pessimistic-Optimist", bans[0].name);
    }
}