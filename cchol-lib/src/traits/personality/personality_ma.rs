pub mod mental_affliction {
    use serde::{Deserialize, Serialize};

    use crate::traits::personality::{BanVec, PersonalityTrait, TraitRollResult};

    #[derive(Debug, Deserialize, Serialize, Clone)]
    pub struct ExtraPersona {
        traits: BanVec,
        exotic_traits: Option<Box<PersonalityTrait>>
    }

    #[derive(Debug, Deserialize, Serialize, Clone)]
    pub enum MentalAffliction {
        SplitPersonality { extras: Vec<ExtraPersona> },
    }

    impl From<MentalAffliction> for PersonalityTrait {
        fn from(ma: MentalAffliction) -> Self {
            match ma {
                MentalAffliction::SplitPersonality { extras } => 
            }
        }
    }

    /// Generate random mental affliction(s).
    pub fn random(bans: &BanVec) -> TraitRollResult {
        
    }
}