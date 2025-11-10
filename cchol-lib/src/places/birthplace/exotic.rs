//! 111: Exotic Birth Location

use std::{collections::HashMap, fs};

use cchol_pm::{HasBiMod, HasName, HasRollRange};
use dicebag::{DiceExt, InclusiveRandomRange, IsOne};
use lazy_static::lazy_static;
use rpgassist::ext::IsNamed;
use serde::{Deserialize, Deserializer, Serialize, de::{self, Visitor}};

use crate::{racial::Race, roll_range::{RollRange, UseRollRange}, serialize::{deserialize_fixed_cr_range, validate_cr_ranges}, skill::native_env::NativeOf, social::{BiMod, CuMod, birth_legitimacy::LegitMod, culture::Culture}, string_manip::resolve_name_hooks};

static EXOTIC_LOCATIONS_FILE: &'static str = "./data/ebloc.json";
lazy_static! {
    static ref EXOTIC_LOCATIONS: Vec<ExoticPlaceOfBirth> = serde_jsonc::from_str(
        &fs::read_to_string(EXOTIC_LOCATIONS_FILE).expect(format!("Error with '{EXOTIC_LOCATIONS_FILE}'?!").as_str())
    ).expect("JSON error");

    static ref EXOTIC_RANGE: RollRange = validate_cr_ranges("EXOTIC_LOCATIONS", &EXOTIC_LOCATIONS, None);
}

static MAX_EPOBALT_CHOICES: usize = 20;
fn dice_size_clamp<'de,D>(deserializer:D) -> Result<usize, D::Error>
where D: Deserializer<'de>
{
    let v: usize = usize::deserialize(deserializer)?;
    match v {
        0 => return Err(de::Error::custom("EPOBAlt `dice_size` cannot be zero!")),
        _ if v > MAX_EPOBALT_CHOICES => return Err(de::Error::custom(format!("Current max `dice_size` for EPOBAlt is {MAX_EPOBALT_CHOICES}"))),
        _ => Ok(v)
    }
}

/// Deserializer for:
/// 
/// ```text
/// "choices": {
///   "#": {
///     "name":"foobar",
///     "bimod+":123
///   },
///   "#": "Barbaz"
/// }
/// ```
fn deserialize_epobalt_choicemap<'de,D>(deserializer:D) -> Result<HashMap<usize, EPOBAltChoice>, D::Error>
where D: Deserializer<'de>
{
    struct NumKeyVisitor;

    #[derive(Deserialize)]
    #[serde(untagged)]
    enum EPOBAltChoiceHalp {
        S(String),
        C(EPOBAltChoice)
    }

    impl <'de> Visitor<'de> for NumKeyVisitor {
        type Value = HashMap<usize, EPOBAltChoice>;
        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a map with numeric string keys")
        }

        fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
        where A: de::MapAccess<'de>,
        {
            let mut hash_map = HashMap::new();
            while let Some(key) = map.next_key::<String>()? {
                hash_map.insert(
                    // usize numeric key for speedy dice roll based map checking...
                    key.parse().map_err(|e| {
                        de::Error::custom(format!("Cannot parse key '{key}' as integer of any sort…: {e}"))
                    })?,
                    // choice between single string or a more complex object…
                    match map.next_value::<EPOBAltChoiceHalp>()? {
                        EPOBAltChoiceHalp::S(name) => EPOBAltChoice { name, bimod: 0, base_environment: None },
                        EPOBAltChoiceHalp::C(epobc) => epobc
                    }
                );
            }
            Ok(hash_map)
        }
    }

    deserializer.deserialize_map(NumKeyVisitor)
}

#[derive(Debug, Deserialize, Serialize, Clone, HasName, HasBiMod)]
pub struct EPOBAltChoice {
    name: String,
    #[serde(rename = "bimod+", default)]
    bimod: i32,
    #[serde(default)]
    base_environment: Option<NativeOf>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct EPOBAlt {
    #[serde(deserialize_with = "dice_size_clamp")] dice_size: usize,
    #[serde(default, deserialize_with = "deserialize_epobalt_choicemap")] choices: HashMap<usize, EPOBAltChoice>,
    #[serde(default)] extends_base: bool,
} impl EPOBAlt {
    pub fn random(&self) -> Option<&EPOBAltChoice> {
        self.choices.get(&1.d(self.dice_size))
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, HasBiMod, HasName, HasRollRange)]
pub struct ExoticPlaceOfBirth {
    bimod: i32,
    name: String,
    alt: Option<EPOBAlt>,
    base_environment: NativeOf,
    #[serde(deserialize_with = "deserialize_fixed_cr_range")] _cr_range: RollRange,
    #[serde(default)] origin_hook: Option<String>,
    #[serde(default)] combined_with: Option<Box<ExoticPlaceOfBirth>>,
} impl ExoticPlaceOfBirth {
    pub fn random(race: &'static Race, culture: &'static Culture, legit: &impl LegitMod) -> Self {
        let roll = (EXOTIC_RANGE.random_of() + culture.cumod() - legit.legitmod())
            .max(*EXOTIC_RANGE.start())
            .min(*EXOTIC_RANGE.end());
        EXOTIC_LOCATIONS.iter()
            .filter(|place| {
                !race.incompatible_with_env(&place.base_environment) &&
                !culture.incompatible_with_env(&place.base_environment)
            })
            .find(|place| place.roll_range().contains(&roll))
            .expect(format!("Err, no suitable exotic location found for '{}' with roll of '{roll}'", race.name()).as_str())
            .clone()
            .resolve(race, culture, legit)
    }

    /// Resolve various things in place…
    fn resolve(mut self, race: &'static Race, culture: &'static Culture, legit: &impl LegitMod) -> Self {
        // check for alt-variations
        if let Some(alt_data) = &self.alt {
            if let Some(alt) = alt_data.random() {
                if alt_data.extends_base {
                    self.name = format!("{} {}", self.name, alt.name());
                } else {
                    self.name = alt.name().into();
                }

                if let Some(env) = &alt.base_environment {
                    self.base_environment = env.clone()
                }

                self.bimod += alt.bimod();
            }
        }

        // Sort out e.g. <Deity> and other such hooks/tags.
        self.name = resolve_name_hooks(self.name(), culture);

        // 10% chance to be a combined place with…
        if 1.d10().is_one() {
            self.bimod += 5;
            // …another exotic place!
            self.combined_with = Some(Box::new(ExoticPlaceOfBirth::random(race, culture, legit)))
        }

        self
    }
}

#[cfg(test)]
mod exotic_birthplace_tests {
    use crate::{places::birthplace::exotic::{EXOTIC_LOCATIONS, ExoticPlaceOfBirth}, racial::Race, roll_range::UseRollRange, social::{BiMod, birth_legitimacy::LegitMod, culture::Culture}};

    #[test]
    fn exotic_place_of_birth_data_integrity() {
        let _ = EXOTIC_LOCATIONS.iter().find(|p| p.roll_range().contains(&1));
    }

    #[test]
    fn foobar() {
        let json = r#"
        {
            "name": "In a brothel",
            "bimod": 2,
            "_cr_range": 4,
            "alt": {
                "dice_size": 5,
                "choices": {
                    "1": {
                        "name": "In a brothel where mother was a prostitute",
                        "bimod+": 3
                    }
                }
            },
            "base_environment": "Urban"
        }"#;
        let epob: ExoticPlaceOfBirth = serde_jsonc::from_str(json).unwrap();
        let race = Race::random();
        let culture = Culture::random(race.max_culture());
        struct Foobar;
        impl LegitMod for Foobar { fn legitmod(&self) -> i32 {0}}
        let legit = Foobar;
        let _ = env_logger::try_init();
        let mut i = 0;
        loop {
            i += 1;
            log::debug!("EPOB attempt #{i}");
            let epob = epob.clone().resolve(race, culture, &legit);
            if 5 == epob.bimod() {
                break;
            }
            if i > 100 {
                panic!("Ok, I give up … 100 rolls of 1d5 … of which NONE resulted in '1'. What gives?")
            }
        }
    }
}