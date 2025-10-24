//! 753: Relatives
use dicebag::DiceExt;
use rpgassist::gender::Gender;
use serde::{Deserialize, Serialize};

use crate::social::family::{AncestryDistance, CousinRelationDistance};

/// Some relatives.
#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum Relation {
    Cousin { distance: CousinRelationDistance, gender: Gender },
    Son,
    Daughter,
    Sister,
    Brother,
    Spouse,
    Aunt { grand_rank: AncestryDistance, related_via: Gender },
    Uncle { grand_rank: AncestryDistance, related_via: Gender },
    // Mother's 'related_via' is relevant only for grand- and more distant relations.
    Mother { grand_rank: AncestryDistance, related_via: Gender },
    // Father's 'related_via' is relevant only for grand- and more distant relations.
    Father { grand_rank: AncestryDistance, related_via: Gender },
    Descendant { generations_removed: i32 },
    ClaimsToBe(Box<Relation>),//TODO: 978#753
}

impl Relation {
    pub fn new() -> Self {
        fn mk_relation(roll_cap: i32) -> Relation {
            match 1.d(roll_cap as usize) {
                ..=1 => Relation::Cousin { distance: CousinRelationDistance::First, gender: Gender::new(None) },
                2 => Relation::Cousin { distance: CousinRelationDistance::Second, gender: Gender::new(None) },
                3 => Relation::Cousin { distance: CousinRelationDistance::Distant, gender: Gender::new(None) },
                4 => Relation::Son,
                5 => Relation::Daughter,
                6 => Relation::Sister,
                7 => Relation::Brother,
                8 => Relation::Spouse,
                9 => Relation::Aunt { grand_rank: AncestryDistance::SiblingOfParent, related_via: Gender::new(None) },
                10 => Relation::Uncle { grand_rank: AncestryDistance::SiblingOfParent, related_via: Gender::new(None) },
                11 => Relation::Aunt { grand_rank: AncestryDistance::Grand, related_via: Gender::new(None) },
                12 => Relation::Uncle { grand_rank: AncestryDistance::Grand, related_via: Gender::new(None) },
                13 => Relation::Mother { grand_rank: AncestryDistance::ChildOf, related_via: Gender::Unspecified },
                14 => Relation::Father { grand_rank: AncestryDistance::ChildOf, related_via: Gender::Unspecified },
                15 => Relation::Mother { grand_rank: AncestryDistance::Grand, related_via: Gender::new(None) },
                16 => Relation::Father { grand_rank: AncestryDistance::Grand, related_via: Gender::new(None) },
                17 => Relation::Mother { grand_rank: AncestryDistance::GreatGrand, related_via: Gender::new(None) },
                18 => Relation::Father { grand_rank: AncestryDistance::GreatGrand, related_via: Gender::new(None) },
                19 => Relation::Descendant { generations_removed: 1.d3() + 1 },
                _ => Relation::ClaimsToBe(Box::new(mk_relation(19)))
            }
        }

        mk_relation(20)
    }
}