//! 106: Family

use dicebag::{DiceExt, IsOne};
use rpgassist::gender::{Gender, HasGender};
use serde::{Deserialize, Serialize};

use crate::{skill::SurvivalMod, social::{CuMod, culture::{Culture, CultureCoreType, HasCultureCoreType}, people::{Relation, guardian::Guardian, relative::{CousinDistance, RelationSubType}}, wealth::Wealth}};

/// Family structure.
#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum FamilyStructure {
    Adopted (Box<FamilyStructure>),
    /// Just mom and dad (+ potential siblings).
    MotherAndFather,
    /// Implicit includes mother and father.
    Extended {
        grandparents: Vec<Relation>,
        // aunts/uncles
        auncles: Vec<Relation>,
        cousins: Vec<Relation>,
    },
    Clan {
        // is one of the clan members The "Mother Figure"
        primary_mother_figure: bool,
        // is one of the clan members The "Father Figure"
        primary_father_figure: bool,
        // These are friends, associates, etc. the character deals/dealt with in a daily basis.
        close_members: u8,
    },
    /// grandpa & grandma
    Grandparents { side: Gender },
    Grandparent { specs: Relation },
    SingleParent { gender: Gender },
    AuntAndUncle { side: Gender },
    AuntOrUncle { specs: Relation },
    Orphanage,
    StreetKid { survival_mod: i32 },
    Guardian(Guardian),
} impl FamilyStructure {
    /// See if family structure affects potential wealth level.
    /// May or may not cause a drop.
    pub fn max_wealth(&self) -> Option<&'static str> {
        match self {
            Self::Orphanage => Some("poor"),
            Self::StreetKid{..} => Some("destitute"),
            _ => None
        }
    }

    /// Generate random [FamilyStructure].
    pub fn random(culture: &Culture) -> Self {
        match 1.d20() + culture.cumod() {
            ..=8 => Self::MotherAndFather,
            ..=12 => match culture.core_type() {
                CultureCoreType::Primitive |
                CultureCoreType::Nomad => Self::Clan {
                    primary_mother_figure: 1.d2().is_one(),
                    primary_father_figure: 1.d2().is_one(),
                    close_members: 3.d4()
                },
                _ => Self::Extended {
                    grandparents: (0..1.d4()).into_iter()
                        .map(|_| RelationSubType::random(RelationSubType::Grandparent, Gender::random(), Gender::random())).collect(),
                    auncles: (0..1.d4()).into_iter()
                        .map(|_| RelationSubType::random(RelationSubType::Auncle, Gender::random(), Gender::random())).collect(),
                    cousins: (0..1.d4()).into_iter()
                        .map(|_|{
                            Relation::Cousin { distance: CousinDistance::First, gender: Gender::random(), side: Gender::random() }
                        }).collect()
                }
            },
            13 => Self::Grandparents { side: Gender::random() },
            14 => Self::Grandparent { specs: RelationSubType::random(RelationSubType::Grandparent, Gender::random(), Gender::random())},
            15 => Self::AuntAndUncle { side: Gender::random() },
            16 => Self::AuntOrUncle { specs: RelationSubType::random(RelationSubType::Auncle, Gender::random(), Gender::random())},
            ..=18 => Self::SingleParent { gender: Gender::Female }/* mom, obviously */,
            19 => Self::SingleParent { gender: Gender::Male }/* dad, ditto */,
            20 => Self::Guardian(Guardian::random(culture)),
            ..=24 => Self::StreetKid { survival_mod: 1.d3() },
            _ => Self::Orphanage
        }
    }
} impl SurvivalMod for FamilyStructure {
    fn survival_mod(&self) -> i32 {
        match self {
            Self::StreetKid { survival_mod } => *survival_mod,
            _ => 0
        }
    }
} impl HasGender for FamilyStructure {
    /// Get [Gender], if applicable for any given [FamilyStructure].
    fn gender(&self) -> Gender {
        match self {
            Self::Adopted(f) => f.gender(),
            Self::AuntAndUncle { .. } => Gender::Unspecified,
            Self::AuntOrUncle { specs } => specs.gender(),
            Self::Clan { .. } => Gender::Unspecified,
            Self::Extended { .. } => Gender::Unspecified,
            Self::Grandparent { specs } => specs.gender(),
            Self::Grandparents { .. } => Gender::Unspecified,
            Self::Guardian(g) => g.gender(),
            Self::MotherAndFather => Gender::Unspecified,
            Self::Orphanage => Gender::NeverApplicable,
            Self::SingleParent { gender } => *gender,
            Self::StreetKid { .. } => Gender::NeverApplicable,
        }
    }
}