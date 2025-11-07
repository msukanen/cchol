use std::fs;

use bitflags::bitflags;
use cchol_pm::{Gendered, HasName};
use dicebag::{DiceExt, IsOne, RandomOf};
use lazy_static::lazy_static;
use rpgassist::{ext::IsNamed, gender::{Gender, HasGender}, resolve::resolve_in_place::ResolveInPlace, serialize::serial_strings::deserialize_strings_to_vec};
use serde::{Deserialize, Deserializer, Serialize, de};

pub mod pet;
use pet::PetAbility;

/// "Hooks" for `_special` field.
#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum AnimalSpecialHandlingHooks {
    Fish,
    Alien,
}

/// Animal variants — basic distinction between wild and tamed.
#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum Animal {
    Wild(AnimalCore),
    Pet(AnimalCore)
} impl Animal {
    /// Tame the animal, if not already a pet.
    pub fn petify(&mut self) {
        if let Animal::Wild(w) = self {
            let mut beast = w.clone();
            beast.pet_abilities = PetAbility::random();
            *self = Animal::Pet(beast)
        }
    }
}

bitflags! {
    /// Environments where any given animal lives/thrives, or which they just commonly utilize.
    #[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq, Eq)]
    #[serde(transparent)]
    pub struct AnimalEnv: u8 {
        const Water  = 1<<0;// 1
        const Land   = 1<<1;// 2
        const Air    = 1<<2;// 4
        const Burrow = 1<<3;// 8
        // Flags which belong to "alien" species ([AnimalSpecialHandlingHooks::Alien]) `resolve()`.
        // DO NOT USE anywhere else, arrite?
        const _Space = 1<<4;// 16
        const _Incorporeal = 1<<5;// 32
    }
}

impl Default for AnimalEnv {
    fn default() -> Self {
        Self::Land
    }
}

fn deserialize_env_from_num<'de,D>(deserializer:D) -> Result<AnimalEnv, D::Error>
where D: Deserializer<'de> {
    let val = u64::deserialize(deserializer)?;
    let val_u8: u8 = val.try_into().map_err(|_| {
        de::Error::custom(format!("Oh come on! We can't make a proper AnimalEnv out of a silly value like '{val}'! Fix it!"))
    })?;
    AnimalEnv::from_bits(val_u8).ok_or_else(|| {
        de::Error::custom(format!("Well, bugger - '{val_u8}' doesn't translate into any combination of AnimalEnv values. Check the code for correct ones..."))
    })
}

fn deserialize_animal_special_hook<'de,D>(deserializer:D) -> Result<Option<AnimalSpecialHandlingHooks>, D::Error>
where D: Deserializer<'de> {
    let s_opt = Option::<String>::deserialize(deserializer)?;
    match s_opt {
        None => Ok(None),
        Some(s) => match s.to_lowercase().as_str() {
            "fish" => Ok(Some(AnimalSpecialHandlingHooks::Fish)),
            "alien" => Ok(Some(AnimalSpecialHandlingHooks::Alien)),
            other => Err(de::Error::custom(format!("Unknown _special hook: '{other}'…")))
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, HasName, Gendered)]
pub struct AnimalCore {
    name: String,
    #[serde(default, deserialize_with="deserialize_strings_to_vec")]
    alt: Vec<String>,
    #[serde(default)] gender: Gender,
    #[serde(default, deserialize_with="deserialize_env_from_num")]
    environment: AnimalEnv,
    #[serde(default)] sab: bool,
    #[serde(default, rename = "_special", deserialize_with = "deserialize_animal_special_hook")]
    special: Option<AnimalSpecialHandlingHooks>,
    #[serde(default)] pet_abilities: Vec<PetAbility>,
} impl AnimalCore {
    /// The animal is an amphibian?
    pub fn is_amphibian(&self) -> bool {
        self.environment.contains(AnimalEnv::Water) && self.environment.contains(AnimalEnv::Land)
    }

    /// The animal stays a baby, forever?
    pub fn stays_a_baby(&self) -> bool {
        self.sab
    }

    /// Preferred enviroment(s)…
    pub fn enviroment(&self) -> AnimalEnv {
        self.environment
    }
}

static ANIMAL_FILE: &'static str = "./data/animal.json";
lazy_static! {
    static ref ANIMALS: Vec<AnimalCore> = {
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum AnimaEntry {
            S(String),
            C(AnimalCore)
        }

        fn anima_shaper(entry: AnimaEntry) -> AnimalCore {
            match entry {
                AnimaEntry::S(name) => AnimalCore {
                    name,
                    alt: vec![],
                    sab: false,
                    environment: AnimalEnv::default(),
                    gender: Gender::Unspecified,
                    special: None,
                    pet_abilities: vec![],
                },
                AnimaEntry::C(a) => a
            }
        }

        let json = &fs::read_to_string(ANIMAL_FILE).expect(format!("Error with '{ANIMAL_FILE}'?!").as_str());
        let items: Vec<AnimaEntry> = serde_jsonc::from_str(json).expect("JSON error");
        items.into_iter().map(anima_shaper).collect()
    };
}

impl Animal {
    /// Generate a random wild animal.
    pub fn random() -> Animal {
        let mut ac = (&ANIMALS.random_of()).clone();
        ac.resolve();
        Animal::Wild(ac)
    }

    /// Generate a random pet.
    pub fn random_pet() -> Animal {
        let mut pet_to_be = Self::random();
        pet_to_be.petify();
        pet_to_be
    }
}

impl ResolveInPlace for AnimalCore {
    fn resolve(&mut self) {
        // Use 'alt' name if such exist and dice so dictate.
        if !self.alt.is_empty() {
            let roll = 1.d(self.alt.len() + 1)-2;
            if roll >= 0 {
                self.name = self.alt[roll as usize].clone();
            }
        }

        // Apply special 'hooks', if applicable.
        if let Some(spc) = &self.special {
            match spc {
                AnimalSpecialHandlingHooks::Fish => {
                    self.environment = AnimalEnv::Water;
                    if 1.d3().is_one() {
                        // mudskipper etc.
                        self.environment |= AnimalEnv::Land
                    }
                },
                AnimalSpecialHandlingHooks::Alien => {
                    self.environment = AnimalEnv::from_bits(1.d(63) as u8).unwrap();
                }
            }
        }
    }
}

#[cfg(test)]
// Don't call PETA even though the mod is called "animal tests"!!!
mod animal_tests {
    use dicebag::RandomOf;

    use crate::animal::ANIMALS;

    #[test]
    fn animal_file_data_integrity() {
        let _ = ANIMALS.random_of();
    }
}