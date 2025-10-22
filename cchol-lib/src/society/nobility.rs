//! 758: Nobility
//! 871: Special Titles for Nobility
//! 
use std::fmt::Display;

use dicebag::DiceExt;
use serde::{Deserialize, Serialize};

use rpgassist::{gender::Gender, modifier::HasModifier};

use crate::{modifier::{CuMod, LitMod, LitModType, TiMod}, society::culture::{CultureLevelType, Culture}};

/// A struct to haul around a barebones Noble NPC.
#[derive(Debug, Clone)]
pub struct NobleNPC {
    pub gender: Gender,
    pub culture: Culture,
    //TODO pub wealth: Wealth,
    pub title: Title,
}

impl NobleNPC {
    /// Generate a barebones throwaway noble NPC which is to be used maybe for a mere notion
    /// and does not have much of any use beyond that.
    /// 
    /// # Args
    /// `opt_c`—some (optional) [CuMod] source.
    pub fn new(opt_c: Option<&impl CuMod>) -> Self {
        let cumod = if let Some(cumod_src) = opt_c { cumod_src.cumod() } else {
            match 1.d20() {
                ..=1 => CultureLevelType::Primitive,
                ..=5 => CultureLevelType::Nomad,
                ..=10 => CultureLevelType::Barbarian,
                ..=17 => CultureLevelType::Civilized,
                _ => CultureLevelType::Decadent
            }.cumod()
        };

        Self {
            gender: Gender::new(None),
            culture: Culture::from(CultureLevelType::from(cumod)),
            //TODO wealth: Wealth::new(...),
            title: Title::new(&CultureLevelType::from(cumod)),
        }
    }
}

/// A full blown nobility descriptor.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Nobility {
    pub title: Title,
    timod: i32,
    /// Land titles — e.g. "Lord Regent", "Marshal of ...", etc. — if any.
    pub land_titles: Option<Vec<String>>,
    /// Land holdings in km², if any.
    pub land_holdings: Option<i32>,
}

/// Noble title.
#[derive(Debug, Deserialize, Serialize, PartialEq, PartialOrd, Clone)]
pub enum Title {
    Hetman,
    Knight,
    /// Prince is a special case; T-758A(†
    Prince {
        /// Parent title. Relevant only when Prince's specs are a fraction of the parents'.
        parent_title: Option<Box<Title>>,
        fraction_owned: f64,
    },
    Baronet,
    Baron,
    Count,
    Earl,
    Subchieftain,
    Jarl,
    Viscount,
    Chieftain,
    Marquis,
    Duke,
    Archduke,
    RoyalPrince,
    Kahn,
    King,
    HighKing,
    Emperor
}

impl Display for Title {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Self::Archduke => "Archduke",
            Self::Baron => "Baron",
            Self::Baronet => "Baronet",
            Self::Chieftain => "Chieftain",
            Self::Count => "Count",
            Self::Duke => "Duke",
            Self::Earl => "Earl",
            Self::Emperor => "Emperor",
            Self::Hetman => "hetman",
            Self::HighKing => "High King",
            Self::Jarl => "Jarl",
            Self::Kahn => "Kahn",
            Self::King => "King",
            Self::Knight => "Sir",
            Self::Marquis => "Marquis",
            Self::Prince {..} |
            Self::RoyalPrince => "Prince",
            Self::Subchieftain => "Subchieftain",
            Self::Viscount => "Viscount",
        })
    }
}

impl Title {
    /// Generate a random noble title based on given [culture type][CultureLevelType].
    /// 
    /// # Args
    /// `cumod_src`— some [CuMod] source.
    pub fn new(cumod_src: &impl CuMod) -> Self {
        match cumod_src.as_clt() {
            CultureLevelType::Primitive =>
                match 1.d100() {
                    ..=1 => Self::HighKing,
                    ..=30 => Self::Chieftain,
                    _ => Self::Subchieftain
                },

            CultureLevelType::Nomad =>
                match 1.d100() {
                    ..=10 => Self::Kahn,
                    ..=40 => Self::Chieftain,
                    ..=80 => Self::Subchieftain,
                    _ => Self::Hetman
                },

            CultureLevelType::Barbarian =>
                match 1.d100() {
                    ..=2 => Self::HighKing,
                    ..=15 => Self::King,
                    ..=25 => Self::RoyalPrince,
                    ..=45 => Self::Chieftain,
                    ..=60 => Self::Jarl,
                    ..=70 => Self::Subchieftain,
                    ..=75 => Self::Baron,
                    ..=80 => Self::mk_prince(cumod_src),
                    _ => Self::Hetman
                },

            CultureLevelType::Civilized |
            CultureLevelType::Decadent =>
                match 1.d100() {
                    ..=1 => Self::Emperor,
                    ..=5 => Self::King,
                    ..=15 => Self::RoyalPrince,
                    ..=20 => Self::Archduke,
                    ..=25 => Self::Duke,
                    ..=35 => Self::Marquis,
                    ..=50 => Self::Viscount,
                    ..=60 => if 1.d2() == 1 {Self::Count} else {Self::Earl},
                    ..=75 => Self::Baron,
                    ..=78 => Self::Baronet,
                    ..=90 => Self::mk_prince(cumod_src),
                    _ => Self::Knight
                }
        }
    }

    /// Generate a prince.
    /// 
    /// # Args
    /// `cumod_src`— some [CuMod] source.
    fn mk_prince(cumod_src: &impl CuMod) -> Self {
        let parent_title = Self::new_prince_parent(cumod_src);
        let fraction_owned = if let Some(_) = parent_title { 1.d10() as f64 * 0.1 } else { 1.0 };
        Self::Prince { parent_title, fraction_owned }
    }

    /// Generate (optional) parents' title for a prince.
    /// 
    /// # Args
    /// `cumod_src`— some [CuMod] source.
    fn new_prince_parent(cumod_src: &impl CuMod) -> Option<Box<Self>> {
        // 1..20 = archduke equivalent, but 21+ parent's title matters instead.
        if 1.d100() > 20 {
            let mut parent = Self::new(cumod_src);
            loop {
                match parent {
                    Self::Prince {..} => {
                        log::debug!("Parent would've been a prince(ss) — redoing…");
                        parent = Self::new(cumod_src);
                    },
                    _ => break None
                }
            }
        } else { None }
    }
}

impl HasModifier for Nobility {
    fn modifier(&self) -> i32 {
        self.timod
    }
}

impl Title {
    /// FYI: for internal/generation-phase use only, not for live querying!
    // NOTE: if the prince(ss)'s parents are prince-equivalent,
    //       the TiMod will be miniscule at best… But so it goes.
    fn modifier(&self) -> i32 {
        match self {
            Self::Emperor   => 60,
            Self::HighKing  => 5.d10(),
            Self::King      => 39,
            Self::Kahn      => 5.d8(),
            Self::RoyalPrince |
            Self::Archduke  => 4.d10(),
            Self::Duke      => 4.d8(),
            Self::Marquis   => 3.d10(),
            Self::Chieftain |
            Self::Jarl      |
            Self::Count     |
            Self::Earl      => 3.d6(),
            Self::Viscount  => 3.d8(),
            Self::Subchieftain |
            Self::Knight    => 2.d6(),
            Self::Baron     => 2.d10(),
            Self::Baronet   => 2.d8(),
            Self::Prince { parent_title: Some(p_title), fraction_owned } => {
                let m = p_title.modifier() as f64;
                (m * fraction_owned) as i32 // NOTE: trunc is fine as rounding is irrelevant.
            },
            Self::Prince {..} => 4.d10(),
            Self::Hetman      => 1.d6()
        }
    }

    /// Generate (potential) land title(s) that the corresponding Title might have.
    fn mk_land_titles(&self) -> Option<Vec<String>> {
        match self {
            Self::Chieftain    |
            Self::Jarl         |
            Self::Subchieftain |
            Self::Prince {..}  |
            Self::Hetman      => None,
            Self::Emperor     => Some(mk_land_titles_vec(1.d4() + 3)),
            Self::HighKing    => Some(mk_land_titles_vec(1.d6())),
            Self::King        => Some(mk_land_titles_vec(1.d4() + 1)),
            Self::Kahn        => Some(mk_land_titles_vec(1.d6())),
            Self::RoyalPrince => Some(mk_land_titles_vec(1.d4())),
            Self::Archduke    => Some(mk_land_titles_vec(1.d3() + 1)),
            Self::Duke        => Some(mk_land_titles_vec(1.d3())),
            Self::Marquis     => Some(mk_land_titles_vec(1.d2())),
            Self::Viscount    => Some(mk_land_titles_vec(1)),
            Self::Count        |
            Self::Earl        => if 1.d100() < 91 {Some(mk_land_titles_vec(1))} else {None},
            Self::Baron       => if 1.d100() < 76 {Some(mk_land_titles_vec(1))} else {None},
            Self::Baronet     => if 1.d100() < 51 {Some(mk_land_titles_vec(1))} else {None},
            Self::Knight      => if 1.d100() < 36 {Some(mk_land_titles_vec(1))} else {None},
        }
    }

    /// Generate land holdings side in km², if any.
    fn mk_land_holdings(&self) -> Option<i32> {
        match self {
            Self::Emperor      => Some(1.d20() * 10),
            Self::HighKing     => if 1.d100() < 86 {Some(1.d20() * 5)} else {None},
            Self::King         => Some(1.d10() * 10),
            Self::Kahn         => if 1.d100() < 31 {Some(1.d10() * 5)} else {None},
            Self::RoyalPrince  => if 1.d100() < 71 {Some(1.d20() * 5)} else {None},
            
            Self::Prince { parent_title: None,.. } |
            Self::Archduke     => if 1.d100() < 76 {Some(1.d10() * 5)} else {None},

            Self::Duke         => if 1.d100() < 86 {Some(1.d10() * 5)} else {None},
            Self::Marquis      => if 1.d100() < 61 {Some(1.d20() + 12)} else {None},
            Self::Chieftain    => if 1.d100() < 41 {Some(2.d6() + 8)} else {None},
            Self::Viscount     => if 1.d100() < 51 {Some(1.d20() + 10)} else {None},
            Self::Jarl         => if 1.d100() < 71 {Some(1.d6() + 4)} else {None},
            Self::Subchieftain => if 1.d100() < 31 {Some(1.d8())} else {None},
            Self::Count |
            Self::Earl         => if 1.d100() < 41 {Some(1.d20() + 4)} else {None},
            Self::Baron        => if 1.d100() < 61 {Some(1.d10() + 4)} else {None},
            Self::Baronet      => if 1.d100() < 31 {Some(1.d10())} else {None},
            Self::Knight       => if 1.d100() < 61 {Some(1.d4())} else {None},
            Self::Hetman       => if 1.d100() < 86 {Some(1.d4())} else {None},
            Self::Prince { parent_title: Some(p_title), fraction_owned } => {
                if let Some(land_holdings) = p_title.mk_land_holdings() {
                    Some((land_holdings as f64 * fraction_owned) as i32)
                } else {
                    None
                }
            },
        }
    }
}

/// Generate solid land title(s) vec.
/// 
/// # Args
/// `count`— number of title(s) to generate.
fn mk_land_titles_vec(count: i32) -> Vec<String> {
    let mut titles = vec![];
    for _ in 0..=count {
        titles.push(mk_land_title());
    }
    titles
}

const LAND_TITLE_PART1: [&'static str; 20] = [
    "Commander",
    "Custodian",
    "Grim Sentinel",
    "High Champion",
    "Honored Defender",
    "Iron Tower",
    "Lord Protector",
    "Liberator",
    "Lord Governor",
    "Lord Guardian",
    "Keeper",
    "Preserver",
    "Marshall",
    "Ranger",
    "Regent",
    "Retaliator",
    "Swordmaster",
    "Vindicator",
    "Warden",
    "Watchwarder",
];
const LAND_TITLE_PART2: [&'static str; 11] = [
    "Highland",
    "Lowland",
    "Upper",
    "Lower",
    "Seaward",
    "Northern",
    "Eastern",
    "Southern",
    "Western",
    "Frozen",
    "Scorched",
];
const LAND_TITLE_PART3: [&'static str; 20] = [
    "Coasts",
    "Creation",
    "Domain",
    "Downs",
    "Fens",
    "Forests",
    "Garth",
    "Heath",
    "Hills",
    "Isles",
    "Marches",
    "Moors",
    "Mountains",
    "Pale",
    "Reaches",
    "Shire",
    "Steppe",
    "Uplands",
    "Wastes",
    "Waves",
];

/// Generate a single land title.
fn mk_land_title() -> String {
    let part1 = LAND_TITLE_PART1[1.d20()-1];
    let r = 1.d(10 + LAND_TITLE_PART2.len());
    if r < 11 { return part1.into() }
    format!("{} of the {} {}",
            part1,
            LAND_TITLE_PART2[r - 11],
            LAND_TITLE_PART3[1.d(LAND_TITLE_PART3.len()) - 1]
        )
}

impl Nobility {
    /// Generate random nobility.
    /// 
    /// # Args
    /// `cumod_src`— some [CuMod] source.
    pub fn new(cumod_src: &impl CuMod) -> Self {
        let title = Title::new(cumod_src);
        let timod = title.modifier();
        let land_titles = title.mk_land_titles();
        let land_holdings = title.mk_land_holdings();

        Self { title, timod, land_titles, land_holdings }
    }


    /// See if we should proceed making a Noble or not.
    /// 
    /// # Args
    /// `cumod_src`— some [CuMod] source.
    pub(crate) fn is_eligible_r(cumod_src: &impl CuMod) -> bool {
        1.d100() + cumod_src.cumod() >= 99
    }
}

impl TiMod for Nobility {
    fn timod(&self) -> i32 {
        self.timod
    }
}

impl LitMod for Nobility {
    fn litmod(&self) -> LitModType {
        // Nobles have +30% literacy chance over the base cultural one.
        LitModType::Additive(30)
    }
}