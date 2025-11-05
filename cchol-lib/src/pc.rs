use std::collections::HashMap;

use rpgassist::{ext::IsNamed, gender::{Gender, HasGender}, stat::{Stat, StatBase}};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::racial::{Race, race::RACES};

fn deserialize_pc_race<'de,D>(deserializer: D) -> Result<&'static Race, D::Error>
where D: Deserializer<'de> {
    let race_name = String::deserialize(deserializer)?;
    Ok(&*RACES.iter()
        .find(|n| n.name() == race_name)
            .expect("JSON error: non-existent Race defined!"))
}

fn serialize_pc_race<S>(race: &&'static Race, serializer: S) -> Result<S::Ok, S::Error>
where S: Serializer {
    serializer.serialize_str(race.name())
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct PlayerCharacter {
    name: String,
    gender: Gender,
    #[serde(deserialize_with = "deserialize_pc_race", serialize_with = "serialize_pc_race")]
    race: &'static Race,
    stats: HashMap<StatBase, Stat>,
}

impl IsNamed for PlayerCharacter {
    fn name(&self) -> &str {
        &self.name
    }
}

impl HasGender for PlayerCharacter {
    fn gender(&self) -> Gender {
        self.gender
    }
}