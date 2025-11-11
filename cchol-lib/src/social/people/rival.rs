//! 762: Rivals
use std::cmp::Ordering;

use dicebag::{DiceExt, IsOne};
use rpgassist::{gender::{Gender, GenderBias, HasGender}, serialize::serial_ordering, ext::IsNamed};
use serde::{Deserialize, Serialize};

use crate::{racial::Race, social::{Deity, people::{OtherPeople, Relation}}, traits::HasCulture};

#[derive(Debug, Deserialize, Serialize, Clone)]
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
    Sibling { gender: Gender,
        #[serde(with = "serial_ordering")]
        relative_age: Ordering },
    Stranger(Box<OtherPeople>),
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
    fn random(culture: &impl HasCulture, potential_deity: bool) -> Self {
        match 1.d10() {
            ..=1 => Self::FormerLover { gender: Gender::random_biased(GenderBias::Female23) },
            2 => Self::FamilyMember(Relation::random()),
            3 => {
                let race = Race::random_nonhuman();
                let gender = race.random_gender();
                Self::Nonhuman { race: race.name().into(), gender }},
            4 => Self::Stranger(Box::new(OtherPeople::random(culture))),
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

/// Some reasons for rivalry. Some more serious than others, some just absurd in a way…
#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum RivalWhy {
    LoveSamePerson,
    SportsEvent,
    ParentsWereRivals,
    CharactersLooks,
    InsultWasPerceived,
    SeekSameGoal,
    Jealousy,
    TryOutdoEachOther,
    DistantAncestorsWereRivals,
    //--- and GM only:
    GM762
} impl RivalWhy {
    fn random() -> Self {
        match 1.d10() {
            ..=1 => Self::LoveSamePerson,
            2 => Self::SportsEvent,
            3 => Self::ParentsWereRivals,
            4 => Self::CharactersLooks,
            5 => Self::InsultWasPerceived,
            6 => Self::SeekSameGoal,
            7 => Self::Jealousy,
            8 => Self::TryOutdoEachOther,
            9 => Self::DistantAncestorsWereRivals,
            _ => Self::GM762
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, PartialOrd)]
pub enum RivalFeelings {
    Friendly,
    Jealous,
    Intense,
    Fierce,
    Deadly,
    Obsessive,
} impl RivalFeelings {
    fn random() -> Self {
        match 1.d10() {
            ..=3 => Self::Friendly,
            4|5 => Self::Jealous,
            6|7 => Self::Intense,
            8 => Self::Fierce,
            9 => Self::Deadly,
            _ => Self::Obsessive
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Rival {
    who: RivalWho,
    why: RivalWhy,
    feeling: RivalFeelings,
} impl HasGender for Rival {
    fn gender(&self) -> Gender {
        self.who.gender()
    }
} impl Rival {
    pub fn random(culture: &impl HasCulture) -> Self {
        Self {
            who: RivalWho::random(culture, false),
            why: RivalWhy::random(),
            feeling: RivalFeelings::random(),
        }
    }
}