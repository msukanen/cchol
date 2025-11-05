pub mod exotic_trait {
    use std::fmt::Display;

    use dicebag::DiceExt;
    use serde::{Deserialize, Serialize};
    use rpgassist::ext::IsNamed;

    use crate::{traits::personality::{TraitVec, TraitRollResult, allergies::{self, Allergy}, behavior_tag::{self, BehaviorTag}, mental_affliction::{self, MentalAffliction}, phobias::{self, Phobia}, sexual_disorder::{self, SexualDisorder}}};

    #[derive(Debug, Deserialize, Serialize, Clone)]
    pub enum ExoticTrait {
        MA(MentalAffliction),
        PH(Phobia),
        AL(Allergy),
        BT(BehaviorTag),
        SD(SexualDisorder),
    }

    impl Display for ExoticTrait {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            unimplemented!();
            match self {
                Self::AL(al) => write!(f, "{al}"),
                Self::BT(bt) => write!(f, "{bt}"),
                Self::MA(ma) => write!(f, "{ma}"),
                Self::PH(ph) => write!(f, "{ph}"),
                Self::SD(sd) => write!(f, "{sd}"),
            }
        }
    }

    /// Generate random exotic feature(s).
    pub fn random(bans: &TraitVec) -> TraitRollResult {
        let mut exs = vec![];
        let mut i = 1;
        while i > 0 {
            i -= 1;
            match 1.d20() {
                ..=4 => exs.extend(mental_affliction::random(bans).as_vec()),
                ..=7 => exs.extend(phobias::random(bans).as_vec()),
                ..=10 => exs.extend(allergies::random(bans).as_vec()),
                ..=17 => exs.extend(behavior_tag::random(bans).as_vec()),
                ..=19 => exs.extend(sexual_disorder::random(bans).as_vec()),
                _ => i += 1.d3() + 1
            }
        }
        TraitRollResult::AddMultiple(exs)
    }

    impl IsNamed for ExoticTrait {
        fn name(&self) -> &str {
            match self {
                Self::AL(x)=> x.name(),
                Self::BT(x)=> x.name(),
                Self::MA(x)=> x.name(),
                Self::PH(x)=> x.name(),
                Self::SD(x)=> x.name()
            }//.proper_case()
        }
    }
}