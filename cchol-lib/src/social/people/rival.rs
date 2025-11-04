//! 762: Rivals
use std::cmp::Ordering;

use dicebag::{DiceExt, IsOne};
use rpgassist::gender::{Gender, GenderBias, HasGender};

use crate::{IsNamed, modifier::CuMod, racial::Race, social::{Deity, culture::Culture, people::{OtherPeople, Relation}}};

/// Who exactly is the rival?
pub enum RivalWho {
    Deity(Deity),
    EnemyOfFamily { gender: Gender },
    FamilyMember(Relation),
    FormerFriend { gender: Gender },
    FormerLover { gender: Gender },
    /// "Rivalry" between friends.
    Friend { gender: Gender },
    Nonhuman {
        // we need just their species' name in this context.
        race: String,
        gender: Gender },
    /// Profession rival is as-is applicable only at teen+ age.
    ProfessionRival { gender: Gender },
    Sibling { gender: Gender, relative_age: Ordering },
    Stranger(OtherPeople),
} impl HasGender for RivalWho {
    fn gender(&self) -> Gender {
        match self {
            Self::Deity(d) => d.gender(),
            Self::FamilyMember(r) => r.gender(),
            Self::EnemyOfFamily { gender } |
            Self::FormerFriend { gender }  |
            Self::FormerLover { gender }   |
            Self::Friend { gender }        |
            Self::Nonhuman { gender,.. }   |
            Self::ProfessionRival { gender}|
            Self::Sibling { gender,.. }    => *gender,
            Self::Stranger(o) => o.gender(),
        }
    }
} impl RivalWho {
    fn random(culture: &Culture, potential_deity: bool) -> Self {
        match 1.d10() {
            ..=1 => Self::FormerLover { gender: Gender::random_biased(GenderBias::Female23) },
            2 => Self::FamilyMember(Relation::random()),
            3 => Self::Nonhuman { race: Race::random_nonhuman().name().into(), gender: Gender::random() },
            4 => Self::Stranger(OtherPeople::random(culture)),
            5 => Self::FormerFriend { gender: Gender::random() },
            6 => Self::EnemyOfFamily { gender: Gender::random() },
            7 => Self::ProfessionRival { gender: Gender::random() },
            8 => Self::Sibling { gender: Gender::random(), relative_age: if 1.d2().is_one() {Ordering::Less} else {Ordering::Greater} },
            9 => Self::Friend { gender: Gender::random() },
            _ => if potential_deity {
                Self::Deity(Deity::random(culture))
            } else {
                Self::random(culture, true)
            }
        }
    }
}

pub struct Rival {
    who: RivalWho,
} impl HasGender for Rival {
    fn gender(&self) -> Gender {
        self.who.gender()
    }
}