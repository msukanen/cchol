pub mod phobias {
    use std::fmt::Display;

    use serde::{Deserialize, Serialize};
    use rpgassist::ext::IsNamed;

    use crate::{traits::personality::{TraitVec, TraitRollResult}};

    #[derive(Debug, Deserialize, Serialize, Clone)]
    pub enum Phobia {

    }

    pub fn random(bans: &TraitVec) -> TraitRollResult {
        unimplemented!()
    }

    impl IsNamed for Phobia {
        fn name(&self) -> &str {
            unimplemented!()
        }
    }

    impl Display for Phobia {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            unimplemented!()
        }
    }
}