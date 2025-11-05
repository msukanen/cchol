use std::cmp::Ordering;

use dicebag::{DiceExt, IsOne};
use rpgassist::{ext::IsNamed, gender::{Gender, HasGender}, serialize::serial_ordering};
use serde::{Deserialize, Serialize};

use crate::{racial::Race, social::{culture::Culture, people::{OtherPeople, Relation, Rival, adventurer::Adventurer}}};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum CompanionWho {
    ChildhoodFriend { gender: Gender },
    FamilyMember(Relation),
    Nonhuman { gender: Gender,
        // need just name of the race in this context.
        race: String },
    Stranger(OtherPeople),
    IntelligentInanimateObject,
    SomeKid { gender: Gender, age_years: u8 },
    Sibling { gender: Gender,
        #[serde(with = "serial_ordering")]
        relative_age: Ordering },
    Adventurer(Adventurer),
    FormerEnemyOrRival(Rival),
    GM761A
} impl HasGender for CompanionWho {
    fn gender(&self) -> Gender {
        match self {
            Self::ChildhoodFriend { gender } |
            Self::Nonhuman { gender,.. } |
            Self::SomeKid { gender,.. } |
            Self::Sibling { gender,.. } => *gender,
            Self::FamilyMember(r) => r.gender(),
            Self::Stranger(s) => s.gender(),
            Self::IntelligentInanimateObject => Gender::NeverApplicable,
            Self::Adventurer(a) => a.gender(),
            Self::FormerEnemyOrRival(f) => f.gender(),
            Self::GM761A => Gender::Unspecified
        }
    }
} impl CompanionWho {
    fn random(culture: &Culture) -> Self {
        match 1.d10() {
            ..=1 => Self::ChildhoodFriend { gender: Gender::random() },
            2 => Self::FamilyMember(Relation::random()),
            3 => {
                let race = Race::random_nonhuman();
                let gender = race.random_gender();
                Self::Nonhuman { race: race.name().into(), gender }},
            4 => Self::Stranger(OtherPeople::random(culture)),
            5 => Self::IntelligentInanimateObject,
            6 => Self::SomeKid { gender: Gender::random(), age_years: 6 + 1.d6() },
            7 => Self::Sibling { gender: Gender::random(), relative_age: if 1.d2().is_one() {Ordering::Less} else {Ordering::Greater}},
            8 => Self::Adventurer(Adventurer::random()),
            9 => Self::FormerEnemyOrRival(Rival::random(culture)),
            _ => if 1.d10().is_one() { Self::GM761A } else { Self::random(culture) }
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum CompanionWhy {
    CharacterSavesTheirLife,
    SeekSimilarGoal { friendly_rivalry_involved: bool },
    ParentsWereCompanions,
    ShareRival(Rival),
    SamePlaceSameTrouble,
    LearningFromCharacter,
    ThieveryFailure/* from the character */,
    ToProtectCharacter,
    MysteriousVoices,
    GM761B
} impl CompanionWhy {
    fn random(culture: &Culture) -> Self {
        match 1.d10() {
            ..=1 => Self::CharacterSavesTheirLife,
            2 => Self::SeekSimilarGoal { friendly_rivalry_involved: 1.d100() < 31 },
            3 => Self::ParentsWereCompanions,
            4 => Self::ShareRival(Rival::random(culture)),
            5 => Self::SamePlaceSameTrouble,
            6 => Self::LearningFromCharacter,
            7 => Self::ThieveryFailure,
            8 => Self::ToProtectCharacter,
            9 => Self::MysteriousVoices,
            _ => Self::GM761B
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum CompanionKind {
    LoyalFriend,
    BumblingBuddy,
    GrimAlly,
    GunghoJoe,
    GroaningGriper,
    GoodOlBoy,
    IncurableRomantic
} impl CompanionKind {
    fn random() -> Self {
        match 1.d10() {
            ..=3 => Self::LoyalFriend,
            4|5 => Self::BumblingBuddy,
            6 => Self::GrimAlly,
            7 => Self::GunghoJoe,
            8 => Self::GroaningGriper,
            9 => Self::GoodOlBoy,
            _ => Self::IncurableRomantic
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Companion {
    who: CompanionWho,
    why: CompanionWhy,
    kind: CompanionKind,
} impl HasGender for Companion {
    fn gender(&self) -> Gender {
        self.who.gender()
    }
} impl Companion {
    pub fn random(culture: &Culture) -> Self {
        Self {
            who: CompanionWho::random(culture),
            why: CompanionWhy::random(culture),
            kind: CompanionKind::random()
        }
    }
}