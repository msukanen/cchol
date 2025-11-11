use dicebag::DiceExt;
use rpgassist::gender::{Gender, HasGender};
use serde::{Deserialize, Serialize};

use crate::Workpad;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum CousinDistance {
    First,
    Second,
    Distant,
} impl Default for CousinDistance {
    fn default() -> Self {
        Self::First
    }
} impl CousinDistance {
    pub fn random() -> CousinDistance {
        match 1.d6() {
            ..=3 => Self::First,
            4|5 => Self::Second,
            _ => Self::Distant
        }
    }
}

/// Relation subtypes — these are not for storing as-is.
pub enum RelationSubType {
    Cousin,
    Auncle,
    GreatAuncle,
    Grandparent,
    GreatGrandparent,
} impl RelationSubType {
    pub fn random(subtype: RelationSubType, individual: Gender, side: Gender) -> Relation {
        match subtype {
            RelationSubType::Cousin => Relation::Cousin { distance: CousinDistance::random(), gender: individual, side },
            RelationSubType::Auncle =>
                if individual == Gender::Female {
                    Relation::Aunt { side }
                } else {
                    Relation::Uncle { side }
                },
            RelationSubType::GreatAuncle =>
                if individual == Gender::Female {
                    Relation::GreatAunt { side }
                } else {
                    Relation::GreatUncle { side }
                },
            RelationSubType::Grandparent =>
                if individual == Gender::Female {
                    Relation::Grandmother { side }
                } else {
                    Relation::Grandfather { side }
                },
            RelationSubType::GreatGrandparent =>
                if individual == Gender::Female {
                    Relation::GreatGrandmother { side }
                } else {
                    Relation::GreatGrandfather { side }
                },
        }
    }

    pub fn random_sibling(workpad: &Workpad) -> Relation {
        if workpad.race().random_gender() == Gender::Male {
            Relation::Brother
        } else {
            Relation::Sister
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum Relation {
    ClaimsToBe(Box<Relation>),
    Cousin { distance: CousinDistance, gender: Gender, side: Gender },
    Son, Daughter,
    Sister, Brother,
    Mother, Father,
    Aunt { side: Gender }, Uncle { side: Gender },
    GreatAunt { side: Gender }, GreatUncle { side: Gender },
    Grandmother { side: Gender }, Grandfather { side: Gender },
    GreatGrandmother {side: Gender }, GreatGrandfather { side: Gender },
    Spouse,
    Descendant { generations_removed: u8, gender: Gender }
} impl Relation {
    pub fn random() -> Self {
        let rel = match 1.d(19) {
            ..=1 => Self::Cousin { distance: CousinDistance::First, gender: Gender::random(), side: Gender::random() },
            2 => Self::Cousin { distance: CousinDistance::Second, gender: Gender::random(), side: Gender::random() },
            3 => Self::Cousin { distance: CousinDistance::Distant, gender: Gender::random(), side: Gender::random() },
            4 => Self::Son,
            5 => Self::Daughter,
            6 => Self::Brother,
            7 => Self::Sister,
            8 => Self::Spouse,
            9 => Self::Aunt { side: Gender::random() },
            10 => Self::Uncle { side: Gender::random() },
            11 => Self::GreatAunt { side: Gender::random() },
            12 => Self::GreatUncle { side: Gender::random() },
            13 => Self::Mother,
            14 => Self::Father,
            15 => Self::Grandmother { side: Gender::random() },
            16 => Self::GreatGrandmother { side: Gender::random() },
            17 => Self::Grandfather { side: Gender::random() },
            18 => Self::GreatGrandfather { side: Gender::random() },
            _ => Self::Descendant { generations_removed: 1.d3()+1, gender: Gender::random() }
        };

        if 1.d20() == 1 {
            Self::ClaimsToBe(Box::new(rel))
        } else {
            rel
        }
    }
}

impl HasGender for Relation {
    fn gender(&self) -> Gender {
        match self {
            Self::Cousin { gender,.. }  |
            Self::Descendant { gender,..}
                              => *gender,
            Self::Brother    |
            Self::Father     |
            Self::Uncle {..} |
            Self::Son        | 
            Self::GreatUncle {..}  |
            Self::Grandfather {..} |
            Self::GreatGrandfather {..}
                              => Gender::Male,
            Self::Aunt  {..}  |
            Self::Mother      |
            Self::Sister      |
            Self::Daughter    |
            Self::GreatAunt  {..}  |
            Self::Grandmother  {..}|
            Self::GreatGrandmother {..}
                              => Gender::Female,
            Self::Spouse      => Gender::Unspecified,
            Self::ClaimsToBe(c) => c.gender()
        }
    }
}