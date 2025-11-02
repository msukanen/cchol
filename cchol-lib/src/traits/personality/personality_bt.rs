pub mod behavior_tag {
    use std::fmt::Display;

    use serde::{Deserialize, Serialize};

    use crate::{IsNamed, traits::personality::{TraitRollResult, TraitVec}};

    #[derive(Debug, Deserialize, Serialize, Clone)]
    pub enum BehaviorTag {

    }

    pub fn random(bans: &TraitVec) -> TraitRollResult {
        unimplemented!()
    }

    impl IsNamed for BehaviorTag {
        fn name(&self) -> String {
            unimplemented!()
        }
    }

    impl Display for BehaviorTag {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            unimplemented!()
        }
    }
}