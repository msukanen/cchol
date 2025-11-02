//! 318A: D/L/N trait chooser.
//! 318B: Neutral Traits
//! 647: Lightside Traits
//! 648: Darkside Traits

use std::fmt::Display;
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
pub use personality_ex::exotic_trait::ExoticTrait;
pub mod personality_ma;
pub use personality_ma::mental_affliction;
pub mod personality_ph;
pub use personality_ph::phobias;
pub mod personality_sd;
pub use personality_sd::sexual_disorder;

pub(crate) type TraitVec = Vec<PersonalityTrait>;

static PERSONALITY_FILE: &'static str = "./data/personality.json";
lazy_static! {
    static ref PERSONALITY_TRAITS: HashMap<String, DLNTrait> = {
        let traits_data: PersonalityTraits = serde_jsonc::from_str(
            &fs::read_to_string(PERSONALITY_FILE)
                .expect(format!("No '{}' found?!", PERSONALITY_FILE).as_str())
        ).expect("JSON error");

        let mut allmap = HashMap::new();

        for mut trait_ in traits_data.lightside_traits {
            trait_.classification = Alignment::L;
            allmap.insert(trait_.name.clone(), trait_);
        }

        for mut trait_ in traits_data.darkside_traits {
            trait_.classification = Alignment::D;
            allmap.insert(trait_.name.clone(), trait_);
        }

        for mut trait_ in traits_data.neutral_traits {
            trait_.classification = Alignment::N;
            allmap.insert(trait_.name.clone(), trait_);
        }

        allmap
    };

    static ref LIGHTSIDE_TRAITS: Vec<&'static DLNTrait> = {
        PERSONALITY_TRAITS.values()
            .filter(|t| t.classification == Alignment::L)
            .collect()
    };

    static ref DARKSIDE_TRAITS: Vec<&'static DLNTrait> = {
        PERSONALITY_TRAITS.values()
            .filter(|t| t.classification == Alignment::D)
            .collect()
    };

    static ref NEUTRAL_TRAITS: Vec<&'static DLNTrait> = {
        PERSONALITY_TRAITS.values()
            .filter(|t| t.classification == Alignment::N)
            .collect()
    };
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct PersonalityTraitExcluder {
    trait_type: String,
    name: String,
}

impl IsNamed for PersonalityTraitExcluder {
    fn name(&self) -> String {
        self.name.clone()
    }
}

/// D/L/N alignment classification (and lack of such distinction).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
pub enum Alignment {
    D,// darkside,
    L,// lightside,
    N,// netural, and…
    NotApplicable// …covers everything else.
}

/// A trait for anything that might affect (N)PC alignment.
pub trait AffectsAlignment {
    /// Get alignment, if applicable.
    fn alignment(&self) -> Alignment;
}

impl Default for Alignment {
    fn default() -> Self {
        Self::NotApplicable
    }
}

/// Various degrees/strength of personality traits.
#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
pub enum TraitStrength {
    Trivial,
    Weak,
    Average,
    Strong,
    Driving,
    Obsessive
}

impl Default for TraitStrength {
    /// Returning [Average][PersonalityTraitStrength::Average] is a fine default…
    fn default() -> Self {
        Self::Average
    }
}

impl TraitStrength {
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

/// D/L/N personality trait.
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DLNTrait {
    name: String,
    #[serde(default)] mutually_excludes: Vec<PersonalityTraitExcluder>,
    #[serde(default)] classification: Alignment,
    #[serde(default)] strength: TraitStrength,
}

impl Display for DLNTrait {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

/// Collective personality trait catcher.
#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum PersonalityTrait {
    DLN(DLNTrait),
    EX(ExoticTrait),
}

impl Display for PersonalityTrait {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DLN(t) => write!(f, "{t}"),
            Self::EX(t) => write!(f, "{t}"),
        }
    }
}

impl IsNamed for PersonalityTrait {
    fn name(&self) -> String {
        match self {
            Self::EX(x) => x.name(),
            Self::DLN(x) => x.name()
        }
    }
}

impl IsNamed for DLNTrait {
    fn name(&self) -> String {
        self.name.clone()
    }
}

/// JSON deserialize struct.
#[derive(Debug, Deserialize)]
struct PersonalityTraits {
    lightside_traits: Vec<DLNTrait>,
    darkside_traits: Vec<DLNTrait>,
    neutral_traits: Vec<DLNTrait>,
}

/// Various types of trait roll results…
pub enum TraitRollResult {
    /// Add a single(?) trait… which may or may not have sub-traits of its own…
    Add(PersonalityTrait),
    /// Add a bunch of traits (or one, if there's just one)…
    AddMultiple(TraitVec),
    /// Signal that the original trait should be evolved into new one instead of adding to list…
    Evolve { what: PersonalityTrait, to: PersonalityTrait },
    /// No match, an error occured, or simply "no-op - nothing to do".
    NoMatch
}

impl TraitRollResult {
    /// Get a simple 'vectorization' of [TraitRollResult]…
    pub fn as_vec(&self) -> Vec<PersonalityTrait> {
        match self {
            Self::Add(t) => vec![(*t).clone()],
            Self::AddMultiple(v) => v
                .iter()
                .map(|v| match v {
                    PersonalityTrait::DLN(v) => PersonalityTrait::DLN(DLNTrait { ..(*v).clone() }),
                    PersonalityTrait::EX(v) => PersonalityTrait::EX((*v).clone())
                })
                .collect::<Vec<PersonalityTrait>>(),
            Self::Evolve { to,.. } => vec![(*to).clone()],
            Self::NoMatch => vec![]
        }
    }
}

/// 318A: Alignment & Attitude
pub fn random(bans: &TraitVec) -> TraitRollResult {
    match 1.d100() {
        ..=50 => TraitRollResult::NoMatch,
        ..=65 => random_neutral(bans),
        ..=80 => random_lightside(bans),
        ..=95 => random_darkside(bans),
        _     => exotic_trait::random(bans)
    }
}

pub fn random_darkside(bans: &TraitVec) -> TraitRollResult { random_dln(bans, &Alignment::D) }
pub fn random_lightside(bans: &TraitVec) -> TraitRollResult { random_dln(bans, &Alignment::L) }
pub fn random_neutral(bans: &TraitVec) -> TraitRollResult { random_dln(bans, &Alignment::N) }

/// 318A: Alignment & Attitude
/// 
/// Generate a random personality trait which doesn't clash with any in the `bans` list.
/// 
/// Some combinations of potential clashes produce an "evolved" trait instead of a simple
/// addition to list…
/// 
/// ```text
///   TraitRollResult::Evolve { what, to }
/// ```
/// 
/// …tells what to replace with what.
#[must_use = "The result tells what else needs to be done, if anything"]
fn random_dln(
        bans: &TraitVec,
        classification: &Alignment
) -> TraitRollResult {
    let pool = match classification {
        Alignment::D => &*DARKSIDE_TRAITS,
        Alignment::L => &*LIGHTSIDE_TRAITS,
        Alignment::N => &*NEUTRAL_TRAITS,
        _ => unimplemented!("TODO")
    };

    let mut bail_out_at_zero = 100;

    loop {
        bail_out_at_zero -= 1;
        
        let entry = pool[1.d(pool.len()) - 1].clone();
        let clash = bans.iter().find(|t|
            t.name() == entry.name() || match t {
                PersonalityTrait::DLN(t) => t.mutually_excludes.iter().any(|x| x.name() == entry.name()),
                _ => false
            });
        match clash {
            None => return TraitRollResult::Add(PersonalityTrait::DLN(entry)),
            Some(clash_with) => {
                log::debug!("Clash: '{}' vs '{}'", clash_with.name(), entry.name());
                if  (clash_with.name() == "Pessimist" && entry.name() == "Optimist")||
                    (clash_with.name() == "Optimist" && entry.name() == "Pessimist") {
                        let evolved_trait = PERSONALITY_TRAITS.get("Pessimistic-Optimist")
                            .expect("DATA ERROR: 'Pessimistic-Optimist' not found!")
                            .clone();
                        return TraitRollResult::Evolve { what: clash_with.clone(), to: PersonalityTrait::DLN(evolved_trait) };
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
    pub fn apply(self, upon: &mut TraitVec) {
        match self {
            Self::Add(x) => upon.push(x),
            Self::AddMultiple(v) => upon.extend(v),
            Self::Evolve { what, to } =>
                *(upon.iter_mut().find(|x| x.name() == what.name()).unwrap()) = to,
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
        bans.push(PersonalityTrait::DLN(PERSONALITY_TRAITS.get("Optimist").unwrap().clone()));
        // lets try until we get TraitRollResult::Evolve
        let mut x = 0;
        let _ = env_logger::try_init();
        loop {
            x += 1;
            log::debug!("Rolling ... {x}");
            let t1 = random_darkside(bans.as_ref());
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
        assert_eq!("Pessimistic-Optimist", bans[0].name());
    }
}