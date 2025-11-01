pub mod sexual_disorder {
    use serde::{Deserialize, Serialize};

    use crate::{IsNamed, traits::personality::{TraitRollResult, TraitVec}};

    #[derive(Debug, Deserialize, Serialize, Clone)]
    pub enum SexualDisorder {

    }

    pub fn random(bans: &TraitVec) -> TraitRollResult {

    }

    impl IsNamed for SexualDisorder {
        
    }
}