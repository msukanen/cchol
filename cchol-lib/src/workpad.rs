//! The middle work-horse to keep tabs on stuff that comes,
//! goes, or gets modified during character generation before
//! all the stuff is summed up and a PlayerCharacter finalized.

use std::{cell::Cell, marker::PhantomData, ops::AddAssign};

use rpgassist::{gender::{Gender, HasGender}, stat::Stat};

use crate::{StatMap, racial::Race, social::{BiMod, CuMod, SolMod, birth::Birth, birth_legitimacy::LegitMod, culture::Culture, nobility::TiMod, status::SocialStatus}};

/// All values in the Workpad are Option<> and accessors will panic if/when
/// accessing something out of preordained sequence.
/// 
/// To ensure that things happen in correct sequence, Workpad is intentionally
/// enforced to be single-thread-only. Don't even try any multithread
/// shenanigans with it, the compiler'll swear at you…
pub struct Workpad {
    _enforced_single_threading: PhantomData<*mut ()>,
    name: Option<String>,
    gender: Option<Gender>,
    statmap: StatMap,
    race: Option<&'static Race>,
    culture: Option<&'static Culture>,
    status: Option<SocialStatus>,
    birth: Option<Birth>,
    // BiMod has to invisibly deal with its one-shot temporary boost, if any.
    // And thus - we use Cell for temp_bimod.
    temp_bimod: Cell<i32>,
} impl Workpad {
    pub fn new() -> Self {
        Self {
            _enforced_single_threading: PhantomData,
            name: None,
            gender: None,
            statmap: StatMap::default(),
            race: None,
            culture: None,
            status: None,
            birth: None,
            // temporaries
            temp_bimod: 0.into(),
        }
    }

    /// Set name.
    pub fn set_name(&mut self, name: &str) {
        if name.is_empty() {
            panic!("Cannot do that, Dave. Empty names are not acceptable…")
        }
        self.name = Some(name.into())
    }

    //---------------------------------
    //
    // A bunch of getters …
    //
    /// Get [Race]
    pub fn get_race(&self) -> &'static Race {
        self.race.as_ref()
            .expect("`Race` not yet determined!")
    }

    /// Get [Culture]
    pub fn get_culture(&self) -> &'static Culture {
        self.culture.as_ref()
            .expect("No can do, `Culture` is still a mystery… Figure that out first, ok?")
    }

    /// Get [SocialStatus]
    pub fn get_social_status(&self) -> &SocialStatus {
        self.status.as_ref()
            .expect("`SocialStatus` is amiss!")
    }

    /// Get [StatMap]
    pub fn get_statmap(&self) -> &StatMap {
        &self.statmap
    }
}

impl HasGender for Workpad {
    fn gender(&self) -> Gender {
        self.gender.as_ref()
            .expect("`Gender` not yet determined!")
            .clone()
    }
}

impl BiMod for Workpad {
    fn bimod(&self) -> i32 {
        let base_bimod = self.birth.as_ref()
            .and_then(|b| Some(b.bimod()))
            .expect("`Birth` not yet happened!");
        let temp_boost = self.temp_bimod.get();
        self.temp_bimod.set(0);
        base_bimod + temp_boost
    }
}

impl CuMod for Workpad {
    fn cumod(&self) -> i32 {
        self.culture.as_ref()
            .and_then(|c| Some(c.cumod()))
            .expect("`Culture` not yet set! Such an uncultured brat…")
    }
}

impl LegitMod for Workpad {
    fn legitmod(&self) -> i32 {
        self.birth.as_ref()
            .and_then(|b| Some(b.legitmod()))
            .expect("`Birth` not yet happened!")
    }
}

impl SolMod for Workpad {
    fn solmod(&self) -> i32 {
        self.status.as_ref()
            .and_then(|s| Some(s.solmod()))
            .expect("`SocialStatus` undefined! Would not matter if we were generating a wild animal, but…")
    }
}

impl TiMod for Workpad {
    fn timod(&self) -> i32 {
        self.status.as_ref()
            .expect("`SocialStatus` undefined! Would not matter if we were generating a wild animal, but…")
            .nobility.as_ref()
            .and_then(|n| Some(n.timod()))
            .unwrap_or(0)
    }
}

/// Workpad += Stat
impl AddAssign<Stat> for Workpad {
    fn add_assign(&mut self, rhs: Stat) {
        self.statmap += rhs
    }
}
/// Workpad += Stat
impl AddAssign<Stat> for &mut Workpad {
    fn add_assign(&mut self, rhs: Stat) {(**self) += rhs}
}

/// Workpad += Gender
impl AddAssign<Gender> for Workpad {
    fn add_assign(&mut self, rhs: Gender) {
        self.gender = Some(rhs)
    }
}
/// Workpad += Gender
impl AddAssign<Gender> for &mut Workpad {
    fn add_assign(&mut self, rhs: Gender) {(**self) += rhs;}
}

/// Workpad += Culture
impl AddAssign<&'static Culture> for Workpad {
    fn add_assign(&mut self, rhs: &'static Culture) {
        self.culture = Some(rhs)
    }
}
/// Workpad += Culture
impl AddAssign<&'static Culture> for &mut Workpad {
    fn add_assign(&mut self, rhs: &'static Culture) {(**self) += rhs;}
}

/// Workpad += Race
impl AddAssign<&'static Race> for Workpad {
    fn add_assign(&mut self, rhs: &'static Race) {
        self.race = Some(rhs)
    }
}
/// Workpad += Race
impl AddAssign<&'static Race> for &mut Workpad {
    fn add_assign(&mut self, rhs: &'static Race) {(**self) += rhs;}
}
