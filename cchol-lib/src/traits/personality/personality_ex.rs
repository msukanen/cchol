pub mod exotic_trait {
    use dicebag::DiceExt;

    use crate::traits::personality::{BanVec, TraitRollResult, allergies, behavior_tag, mental_affliction, phobias, sexual_disorder};

    /// Generate random exotic feature(s).
    pub fn random(bans: &BanVec) -> TraitRollResult {
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
}