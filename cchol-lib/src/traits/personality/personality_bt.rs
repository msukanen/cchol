pub mod behavior_tag {
    use serde::{Deserialize, Serialize};

    use crate::{IsNamed, traits::personality::{TraitRollResult, TraitVec}};

    #[derive(Debug, Deserialize, Serialize, Clone)]
    pub enum BehaviorTag {

    }

    pub fn random(bans: &TraitVec) -> TraitRollResult {

    }

    impl IsNamed for BehaviorTag {
        
    }
}