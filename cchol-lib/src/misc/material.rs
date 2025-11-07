use std::collections::HashSet;

use dicebag::DiceExt;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Hash)]
pub enum Substance {
    Granite, Marble,
    Wood, IronWood,
    PreciousMetal,
    Cloth,
    Gemstone,
    Iron,
    Bronze
} impl Substance {
    pub fn random() -> HashSet<Self> {
        let mut set = HashSet::new();
        let mut i = 1;
        while i > 0 && set.len() < 9 {
            i -= 1;
            let matter = match 1.d10() {
                ..=1 => Self::Granite,
                2 => Self::Marble,
                3 => Self::Wood,
                4 => Self::IronWood,
                5 => Self::PreciousMetal,
                6 => Self::Cloth,
                7 => Self::Gemstone,
                8 => Self::Iron,
                9 => Self::Bronze,
                _ => {
                    i += 2;
                    continue;
                }
            };
            if set.contains(&matter) {
                i += 1;
                continue;
            }
            set.insert(matter);
        }
        set
    }
}