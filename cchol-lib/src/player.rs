use rpgassist::gender::Gender;

use crate::{modifier::{CuMod, SolMod}, named::IsNamed, racial::race::Race, society::{culture::{Culture, CultureLevelType}, status::SocialStatus}};

pub struct PlayerCharacter {
    pub gender: Gender,
    name: String,
    race: Race,
    culture: Culture,
    status: SocialStatus,
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

    /// Generate a player character's background from grounds up.
    pub fn new(name: &str, gender: Option<Gender>, race: Option<Race>) -> Self {
        let race = race.unwrap_or_else(|| Race::new());
        let gender = race.readjust_gender(gender.unwrap_or_else(|| race.rnd_gender()));
        let culture = race.culture_shift_if_needed(Culture::new(Some(&race)));
        let status = SocialStatus::new(&culture);

        Self {
            gender,
            name: name.to_string(),
            race,
            culture,
            status,
        }
    }
}

impl CuMod for PlayerCharacter {
    fn cumod(&self) -> i32 {
        self.culture.cumod()
    }
}

impl SolMod for PlayerCharacter {
    fn solmod(&self) -> i32 {
    }
}