use rpgassist::gender::Gender;
use serde::{Deserialize, Serialize};

use crate::{misc::time_of_year::TimeOfBirth, modifier::{CuMod, SolMod}, named::IsNamed, racial::race::Race, society::{birth::{determine_birth_legitimacy, determine_birth_order, determine_date_of_birth, BirthLegitimacy, BirthOrder}, culture::Culture, family::{determine_siblings, FamilyMember}, status::SocialStatus}};

#[derive(Debug, Deserialize, Serialize)]
pub struct PlayerCharacter {
    pub gender: Gender,
    name: String,
    race: Race,
    culture: Culture,
    status: SocialStatus,
    birth_legit: BirthLegitimacy,
    siblings: Option<Vec<FamilyMember>>,
    birth_order: Option<BirthOrder>,
    time_of_birth: TimeOfBirth,
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
        let birth_legit = determine_birth_legitimacy(&culture);
        // We determine siblings only if birth was legit.
        let siblings = determine_siblings(&birth_legit);
        // Birth order. Determined only if there's a reason to …
        let birth_order = determine_birth_order(siblings.as_ref());
        let time_of_birth = determine_date_of_birth(&birth_legit);

        Self {
            name: name.to_string(),
            gender,
            race,
            culture, status,
            birth_legit,
            siblings, birth_order,
            time_of_birth,
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
        self.status.solmod()
    }
}