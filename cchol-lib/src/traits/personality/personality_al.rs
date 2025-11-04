pub mod allergies {
    use std::fmt::Display;

    use serde::{Deserialize, Serialize};

    use crate::{IsNamed, traits::personality::{TraitRollResult, TraitVec}};

    #[derive(Debug, Deserialize, Serialize, Clone)]
    pub enum Allergy {

    }

    pub fn random(bans: &TraitVec) -> TraitRollResult {
        unimplemented!()
    }

    impl IsNamed for Allergy {
        fn name(&self) -> &str {
            unimplemented!()
        }
    }

    impl Display for Allergy {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            unimplemented!()
        }
    }
}