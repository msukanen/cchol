use dicebag::DiceExt;
use rpgassist::gender::{Gender, HasGender};
use serde::{Deserialize, Serialize};

use crate::{modifier::CuMod, society::birth::BirthLegitimacy};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum CousinRelationDistance {
    First,
    Second,
    Distant
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum AncestryDistance {
    Descendant,
    ChildOf,
    // SiblingOfParent is largely relevant only for FamilyMember::Aunt/Uncle.
    SiblingOfParent,
    Grand,
    GreatGrand,
    GreatGreatGrand,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum FamilyMember {
    // Mother's 'related_via' is relevant only for grand- and more distant relations.
    Mother {grand_rank: AncestryDistance, related_via: Gender},
    // Father's 'related_via' is relevant only for grand- and more distant relations.
    Father {grand_rank: AncestryDistance, related_via: Gender},
    Aunt {grand_rank: AncestryDistance, related_via: Gender},
    Uncle {grand_rank: AncestryDistance, related_via: Gender},
    Cousin {distance: CousinRelationDistance, gender: Gender},
    //TODO: Guardian,
    Sibling {gender: Gender, birth_legit: bool},
}

impl HasGender for FamilyMember {
    fn gender(&self) -> rpgassist::gender::Gender {
        match self {
            Self::Aunt {..}   |
            Self::Mother {..} => Gender::Female,
            
            Self::Father {..} |
            Self::Uncle {..}  => Gender::Male,
            
            Self::Cousin { gender,.. } |
            Self::Sibling { gender,..} => *gender,
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum Family {
    /// Mother + father.
    Parents,
    /// Extended family (and implies [Family::Parents]).
    Extended {
        grandparents: Vec<FamilyMember>,
        aunts_and_uncles: Vec<FamilyMember>,
        cousins: Vec<FamilyMember>,
    },
    Grandparents {related_via: Gender},
    Grandparent {gender: Gender, related_via: Gender},
    AuntAndUncle {related_via: Gender},
    Aunt {related_via: Gender},
    Uncle {related_via: Gender},
    Mother,
    Father,
    //TODO: Guardian
    // None known: left to fend for self.
    NoneKnown,
    // None known: raised in an orphanage.
    Orphanage,
}

/// Generate random family, or lack of such.
/// 
/// # Args
/// `cumod_src`— some [CuMod] source.
pub fn generate_family(cumod_src: &impl CuMod) -> Family {
    match 1.d20() + cumod_src.cumod() {
        ..=8 => Family::Parents,
        ..=12 => Family::Extended {
            grandparents: {
                let mut gps = vec![];
                for _ in 0..1.d4() {
                    let related_via = Gender::new(None);
                    gps.push(if 1.d2() == 1 {
                        FamilyMember::Father { grand_rank: AncestryDistance::Grand, related_via }
                    } else {
                        FamilyMember::Mother { grand_rank: AncestryDistance::Grand, related_via }
                    });
                }
                gps
            },
            aunts_and_uncles: {
                let mut anu = vec![];
                for _ in 0..1.d4() {
                    let related_via = Gender::new(None);
                    anu.push(if 1.d2() == 1 {
                        FamilyMember::Uncle { grand_rank: AncestryDistance::SiblingOfParent, related_via }
                    } else {
                        FamilyMember::Aunt { grand_rank: AncestryDistance::SiblingOfParent, related_via }
                    });
                }
                anu
            },
            cousins: {
                let mut cs = vec![];
                for _ in 0..1.d4() {
                    cs.push(FamilyMember::Cousin { distance: CousinRelationDistance::First, gender: Gender::new(None) });
                }
                cs
            }
        },
        13 => Family::Grandparents { related_via: Gender::new(None) },
        14 => Family::Grandparent { gender: Gender::new(None), related_via: Gender::new(None) },
        15 => Family::AuntAndUncle { related_via: Gender::new(None) },
        16 => {
            let related_via = Gender::new(None);
            if 1.d2() == 1 {
                Family::Uncle { related_via }
            } else {
                Family::Aunt { related_via }
            }
        },
        ..=18 => Family::Mother,
        19 => Family::Father,
        //TODO: Guardian as '20'
        ..=24 => Family::NoneKnown,
        _ => Family::Orphanage
    }
}

/// Determine siblings, if any. Siblings are generally unknown in case where the character's
/// birth was not "legitimate".
/// 
/// # Args
/// `birth_legit`— optional birth illegitimacy.
pub fn determine_siblings(birth_legit: &BirthLegitimacy) -> Option<Vec<FamilyMember>> {
    if birth_legit.is_legitimate() {
        return None
    }

    let mut sibs = vec![];
    let mut illegit_count = 0;
    let mut count = 0;
    let count = loop {
        match 1.d20() {
            ..=2 => break count,
            ..=9 => count += 1.d3(),
            ..=15 => count += 1.d3() + 1,
            ..=17 => count += 1.d4() + 2,
            ..=19 => count += 2.d4(),
            _ => illegit_count += 1.d3()
        }
    };

    for _ in 0..count {
        sibs.push(FamilyMember::Sibling { gender: Gender::new(None), birth_legit: true });
    }

    for _ in 0..illegit_count {
        sibs.push(FamilyMember::Sibling { gender: Gender::new(None), birth_legit: false });
    }

    Some(sibs)
}