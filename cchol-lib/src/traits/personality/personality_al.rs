pub mod allergies {
    use serde::{Deserialize, Serialize};

    use crate::{IsNamed, traits::personality::{TraitRollResult, TraitVec}};

    #[derive(Debug, Deserialize, Serialize, Clone)]
    pub enum Allergy {

    }

    pub fn random(bans: &TraitVec) -> TraitRollResult {

    }

    impl IsNamed for Allergy {
        
    }
}