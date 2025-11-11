//! The middle work-horse to keep tabs on stuff that comes,
//! goes, or gets modified during character generation before
//! all the stuff is summed up and a PlayerCharacter finalized.

use std::{cell::Cell, marker::PhantomData, ops::AddAssign};

use rpgassist::{ext::IsNamed, gender::{Gender, HasGender}, stat::Stat};

use crate::{StatMap, modifier::{BiMod, CuMod, LegitMod, SolMod, TiMod}, racial::Race, social::{birth::Birth, culture::{Culture, CultureCoreType}, status::SocialStatus}, traits::{HasCulture, HasCultureCoreType}};

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
    pub fn race(&self) -> &'static Race {
        self.race.as_ref()
            .expect("`Race` not yet determined!")
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

    /// Get [Birth]
    pub fn get_birth(&self) -> &Birth {
        self.birth.as_ref()
            .expect("`Birth` hasn't happened yet!")
    }

    //---------------------------------
    //
    // "Boosters"
    //
    /// One-shot boost to **BiMod** — lasts until next call to `.bimod()`.
    /// 
    /// **NOTE** that `boost_bimod()` is chainable.
    pub fn boost_bimod(&mut self, by: i32) -> &mut Self {
        self.temp_bimod = by.into();
        self
    }
}

impl HasGender for Workpad {
    fn gender(&self) -> Gender {
        self.gender.as_ref()
            .expect("`Gender` not yet determined!")
            .clone()
    }
} impl HasGender for &mut Workpad {/*delegate*/fn gender(&self) -> Gender {(**self).gender()}}

impl BiMod for Workpad {
    fn bimod(&self) -> i32 {
        let base_bimod = self.birth.as_ref()
            .and_then(|b| Some(b.bimod()))
            .expect("`Birth` not yet happened!");
        
        // one-shot temporary booster, used e.g. for UBO rerolls for higher rerolled value…
        let temp_boost = self.temp_bimod.get();
        self.temp_bimod.set(0);

        base_bimod + temp_boost
    }
} impl BiMod for &mut Workpad {/*delegate*/fn bimod(&self) -> i32 {(**self).bimod()}}

impl CuMod for Workpad {
    fn cumod(&self) -> i32 {
        self.culture.as_ref()
            .and_then(|c| Some(c.cumod()))
            .expect("`Culture` not yet set! Such an uncultured brat…")
    }
} impl CuMod for &mut Workpad {/*delegate*/fn cumod(&self) -> i32 {(**self).cumod()}}

impl LegitMod for Workpad {
    fn legitmod(&self) -> i32 {
        self.birth.as_ref()
            .and_then(|b| Some(b.legitmod()))
            .expect("`Birth` not yet happened!")
    }
} impl LegitMod for &mut Workpad {/*delegate*/fn legitmod(&self) -> i32 {(**self).legitmod()}}

impl SolMod for Workpad {
    fn solmod(&self) -> i32 {
        self.status.as_ref()
            .and_then(|s| Some(s.solmod()))
            .expect("`SocialStatus` undefined! Would not matter if we were generating a wild animal, but…")
    }
} impl SolMod for &mut Workpad {/*delegate*/fn solmod(&self) -> i32 {(**self).solmod()}}

impl TiMod for Workpad {
    fn timod(&self) -> i32 {
        self.status.as_ref()
            .expect("`SocialStatus` undefined! Would not matter if we were generating a wild animal, but…")
            .nobility.as_ref()
            .and_then(|n| Some(n.timod()))
            .unwrap_or(0)
    }
} impl TiMod for &mut Workpad {/*delegate*/fn timod(&self) -> i32 {(**self).timod()}}

//-------------------------------------
/// Workpad += Stat
impl AddAssign<Stat> for Workpad {
    fn add_assign(&mut self, rhs: Stat) {
        self.statmap += rhs
    }
} impl AddAssign<Stat> for &mut Workpad {/*delegate*/fn add_assign(&mut self, rhs: Stat) {**self += rhs}}

//-------------------------------------
/// Workpad += Gender
impl AddAssign<Gender> for Workpad {
    fn add_assign(&mut self, rhs: Gender) {
        self.gender = Some(rhs)
    }
} impl AddAssign<Gender> for &mut Workpad {/*delegate*/fn add_assign(&mut self, rhs: Gender) {**self += rhs;}}

//-------------------------------------
/// Workpad += Culture
impl AddAssign<&'static Culture> for Workpad {
    fn add_assign(&mut self, rhs: &'static Culture) {
        self.culture = Some(rhs)
    }
} impl AddAssign<&'static Culture> for &mut Workpad {/*delegate*/fn add_assign(&mut self, rhs: &'static Culture) {**self += rhs;}}

//-------------------------------------
/// Workpad += Race
impl AddAssign<&'static Race> for Workpad {
    fn add_assign(&mut self, rhs: &'static Race) {
        self.race = Some(rhs)
    }
} impl AddAssign<&'static Race> for &mut Workpad {/*delegate*/fn add_assign(&mut self, rhs: &'static Race) {**self += rhs;}}

//-------------------------------------
/// Workpad += SocialStatus
impl AddAssign<SocialStatus> for Workpad {
    fn add_assign(&mut self, rhs: SocialStatus) {
        self.status = Some(rhs)
    }
} impl AddAssign<SocialStatus> for &mut Workpad {/*delegate*/fn add_assign(&mut self, rhs: SocialStatus) {**self += rhs}}

//-------------------------------------
/// Workpad += Birth
impl AddAssign<Birth> for Workpad {
    fn add_assign(&mut self, rhs: Birth) {
        self.birth = Some(rhs)
    }
} impl AddAssign<Birth> for &mut Workpad {/*delegate*/fn add_assign(&mut self, rhs: Birth) {**self += rhs}}

impl IsNamed for Workpad {
    fn name(&self) -> &str {
        self.name.as_ref()
            .expect("No `name` set yet! Go get a name for yourself!")
    }
} impl IsNamed for &mut Workpad {/*delegate*/fn name(&self) -> &str {(**self).name()}}

//
// Culture related stuff.
//
impl HasCulture for Workpad {
    fn culture(&self) -> &'static Culture {
        self.culture.as_ref()
            .expect("No can do, `Culture` is still a mystery… Figure that out first, ok?")
    }
} impl HasCulture for &mut Workpad {/*delegate*/fn culture(&self) -> &'static Culture {(**self).culture()}}

impl HasCultureCoreType for Workpad {
    fn core_type(&self) -> &'static CultureCoreType {
        self.culture().core_type()
    }
} impl HasCultureCoreType for &mut Workpad {/*delegate*/fn core_type(&self) -> &'static CultureCoreType {(**self).core_type()}}
