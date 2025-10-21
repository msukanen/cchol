//! 102: Culture
use dicebag::DiceExt;
use serde::{Deserialize, Serialize};

use crate::{modifier::{CuMod, LitModType, SurvivalModNatEnv}, racial::race::Race, skill::literacy::{HasLiteracyBenefit, PotentialLanguage}, society::environment::NativeEnvironment};

/// Culture level types for internal matcharoo.
#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum CultureLevelType {
    Primitive, Nomad, Barbarian, Civilized, Decadent
}

impl From<Level> for CultureLevelType {
    fn from(value: Level) -> Self {
        match value {
            Level::Primitive => Self::Primitive,
            Level::Nomad => Self::Nomad,
            Level::Barbarian(_) => Self::Barbarian,
            Level::Civilized(_) => Self::Civilized,
            Level::Decadent => Self::Decadent
        }
    }
}

impl From<&Level> for CultureLevelType {
    fn from(value: &Level) -> Self {
        Self::from(value.clone())
    }
}

/// Various culture levels.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize, Clone, Copy)]
pub enum Level {
    Primitive,
    Nomad,
    Barbarian(NativeEnvironment),
    Civilized(NativeEnvironment),
    Decadent
}

impl CuMod for CultureLevelType {
    fn cumod(&self) -> i32 {
        match self {
            Self::Primitive => -3,
            Self::Nomad => 0,
            Self::Barbarian => 2,
            Self::Civilized => 4,
            Self::Decadent => 7
        }
    }
}

impl CuMod for Level {
    fn cumod(&self) -> i32 {
        CultureLevelType::from(self).cumod()
    }
}

impl Level {
    /// Generate random culture level; [`race`][Race] (if given) dictates maximum culture level, etc.
    /// 
    /// # Params
    /// * `race` — (optional) some [Race].
    pub fn new(race: Option<&Race>) -> Self {
        let mut culture = match 1.d10() {
            ..=1 => Self::Primitive,
            2|3 => Self::Nomad,
            4..=6 => Self::Barbarian(NativeEnvironment::new(Some(NativeEnvironment::Wilderness))),
            7..=9 => Self::Civilized(NativeEnvironment::new(Some(NativeEnvironment::Urban))),
            _ => Self::Decadent
        };
        
        if let Some(race) = race {
            if let Some(max_culture) = race.max_culture() {
                if max_culture < culture.into() {
                    culture = Level::from(max_culture)
                }
            }
            culture = race.culture_shift_if_needed(culture)
        }
        culture
    }

    /// Get culture's [native environment][NativeEnvironment].
    pub fn native_env(&self) -> NativeEnvironment {
        match self {
            Self::Barbarian(e) |
            Self::Civilized(e) => *e,
            Self::Decadent => NativeEnvironment::Urban,
            _ => NativeEnvironment::Wilderness
        }
    }
}

impl PartialEq<CultureLevelType> for Level {
    fn eq(&self, other: &CultureLevelType) -> bool {
        CultureLevelType::from(*self) == *other
    }
}

impl From<CultureLevelType> for Level {
    /// Generate [culture level][Level] from `value`. Some randomness is bound to happen with
    /// [native environment][NativeEnvironment] for
    /// [barbarian][CultureLevelType::Barbarian] and
    /// [civilized][CultureLevelType::Civilized].
    fn from(value: CultureLevelType) -> Self {
        match value {
            CultureLevelType::Primitive => Self::Primitive,
            CultureLevelType::Nomad => Self::Nomad,
            // Bias is "hard coded" for Barbies and Civies as [From] doesn't let us have many params …
            CultureLevelType::Barbarian => Self::Barbarian(NativeEnvironment::new(Some(NativeEnvironment::Wilderness))),
            CultureLevelType::Civilized => Self::Civilized(NativeEnvironment::new(Some(NativeEnvironment::Urban))),
            CultureLevelType::Decadent => Self::Decadent
        }
    }
}

impl From<(CultureLevelType, NativeEnvironment)> for Level {
    /// Generate [culture level][Level] from `value.0` while `value.1` holds on to
    /// [native environment][NativeEnvironment] bias (which might or might not be used/needed).
    fn from(value: (CultureLevelType, NativeEnvironment)) -> Self {
        match value.0 {
            CultureLevelType::Primitive => Self::Primitive,
            CultureLevelType::Nomad => Self::Nomad,
            CultureLevelType::Barbarian => Self::Barbarian(NativeEnvironment::new(Some(value.1))),
            CultureLevelType::Civilized => Self::Civilized(NativeEnvironment::new(Some(value.1))),
            CultureLevelType::Decadent => Self::Decadent
        }
    }
}

impl SurvivalModNatEnv for Level {
    fn survmod_in_natenv(&self, native_env: &NativeEnvironment) -> i32 {
        match self {
            Self::Primitive => match native_env {
                NativeEnvironment::Wilderness => 5, _=> 1,
            },

            Self::Nomad => match native_env {
                NativeEnvironment::Wilderness => 5, _=> 1,
            },

            Self::Barbarian(e) => match native_env {
                _ if native_env == e => 5,
                _ => 1
            },

            Self::Civilized(e) => match native_env {
                _ if native_env == e => 2,
                _ => 0
            },

            Self::Decadent => match native_env {
                NativeEnvironment::Urban => 3, _=> 1
            }
        }
    }
}

impl HasLiteracyBenefit for CultureLevelType {
    fn literacy(&self) -> Vec<PotentialLanguage> {
        match self {
            Self::Primitive => vec![
                PotentialLanguage::new("Foreign language", LitModType::Additive(5))
            ],

            Self::Nomad => vec![
                PotentialLanguage::new("Native pictographs", LitModType::FixedOverride(100)),
                PotentialLanguage::new("Foreign pictographs", LitModType::Additive(10)),
                PotentialLanguage::new("Foreign language", LitModType::Additive(10)),
            ],

            Self::Barbarian => vec![
                PotentialLanguage::new("Native language", LitModType::Additive(10)),
            ],

            Self::Civilized => vec![
                PotentialLanguage::new("Native language", LitModType::Additive(30)),
            ],

            Self::Decadent => vec![
                PotentialLanguage::new("Native language", LitModType::Additive(20)),
                PotentialLanguage::new("Foreign language", LitModType::Additive(10))
            ]
        }
    }
}

#[cfg(test)]
mod culture_tests {
    use crate::{racial::race::Race, society::{culture::{CultureLevelType, Level}, environment::NativeEnvironment}};

    #[test]
    fn reptileman_shift_nomad_down() {
        let r = Race::Reptileman;
        let culture = Level::Nomad;
        let culture = r.culture_shift_if_needed(culture);
        assert_eq!(CultureLevelType::Primitive, culture.into());
    }

    #[test]
    fn reptileman_shift_civilized_up() {
        let r = Race::Reptileman;
        let culture = Level::Civilized(NativeEnvironment::Urban);
        let culture = r.culture_shift_if_needed(culture);
        assert_eq!(CultureLevelType::Decadent, culture.into());
    }
}