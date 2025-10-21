use rpgassist::gender::Gender;

use crate::{named::IsNamed, racial::race::Race};

pub struct PlayerCharacter {
    pub gender: Gender,
    name: String,
    race: Race,
}

impl IsNamed for PlayerCharacter {
    fn name(&self) -> &str {
        &self.name
    }
}

impl PlayerCharacter {
    pub fn race(&self) -> &Race {
        &self.race
    }

    pub fn new(name: &str, gender: Option<Gender>, race: Option<Race>) -> Self {
        let race = race.unwrap_or_else(|| Race::new());
        let gender = gender.unwrap_or_else(|| race.rnd_gender());

        Self {
            gender,
            name: name.to_string(),
            race
        }
    }
}