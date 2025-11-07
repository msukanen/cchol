use std::collections::HashSet;

use dicebag::DiceExt;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use strum::{EnumIter, IntoEnumIterator};

lazy_static! {
    static ref SUBSTANCE_COUNT: usize = Substance::iter().count();
}
/// Various substances. Some more exotic/unusual than others…
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, Hash, EnumIter)]
pub enum Substance {
    Granite, Marble, Wood, IronWood, PreciousMetal,
    Cloth, Gemstone, Iron, Bronze
} impl Substance {
    pub fn random() -> HashSet<Self> {
        let mut set = HashSet::new();
        let mut i = 1;
        // loop around as long as we have to and we've no exhausted ALL options...
        while i > 0 && set.len() < *SUBSTANCE_COUNT {
            i -= 1;
            let roll = 1.d(*SUBSTANCE_COUNT + 1);
            if roll > *SUBSTANCE_COUNT {
                // more than 1 material
                i += 2;
                continue;
            }

            let matter = Substance::iter().nth(roll - 1).unwrap();
            // reroll if already present.
            if set.contains(&matter) {
                i += 1;
                continue;
            }
            set.insert(matter);
        }
        set
    }
}