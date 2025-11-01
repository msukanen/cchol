pub mod phobias {
    use serde::{Deserialize, Serialize};

    use crate::{IsNamed, traits::personality::{TraitVec, TraitRollResult}};

    #[derive(Debug, Deserialize, Serialize, Clone)]
    pub enum Phobia {

    }

    pub fn random(bans: &TraitVec) -> TraitRollResult {

    }

    impl IsNamed for Phobia {
        
    }
}