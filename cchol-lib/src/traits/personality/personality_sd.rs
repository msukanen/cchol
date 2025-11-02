pub mod sexual_disorder {
    use std::fmt::Display;

    use serde::{Deserialize, Serialize};

    use crate::{IsNamed, traits::personality::{TraitRollResult, TraitVec}};

    #[derive(Debug, Deserialize, Serialize, Clone)]
    pub enum SexualDisorder {

    }

    pub fn random(bans: &TraitVec) -> TraitRollResult {
        unimplemented!("")
    }

    impl IsNamed for SexualDisorder {
        fn name(&self) -> String {
            unimplemented!()
        }
    }

    impl Display for SexualDisorder {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            unimplemented!()
        }
    }
}