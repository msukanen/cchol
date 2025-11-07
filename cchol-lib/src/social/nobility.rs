//! 103: Social Status
//! 758: Nobles

use std::fs;
use cchol_pm::{Gendered, HasRollRange};
use lazy_static::lazy_static;
use rpgassist::{gender::{Gender, GenderBias, HasGender}, ext::IsNamed, serialize::serial_strings::deserialize_strings_to_vec};
use serde::{Deserialize, Serialize};
use dicebag::{DiceExt, InclusiveRandomRange, percentage_chance_of};

use crate::{misc::ConditionalExec, roll_range::*, serialize::{deserialize_cr_range, deserialize_string_w_optional}, social::culture::{Culture, CultureCoreType, HasCultureCoreType}};

static NOBLENOTES_FILE: &'static str = "./data/nobility.json";
static NOBLE_TITLE_PARTS_FILE: &'static str = "./data/land_titles.json";
lazy_static! {
    // Load and parse NobleNotes …
    static ref NOBILITYFILE: NobilityFile = {
        serde_jsonc::from_str(
            &fs::read_to_string(NOBLENOTES_FILE)
                .expect(format!("No '{}' found?!", NOBLENOTES_FILE).as_str())
        ).expect("JSON error")
    };

    // Accessor for NobilityFile.titles …
    static ref NOBLENOTES: &'static Vec<NobleNote> = &NOBILITYFILE.titles;

    // Determine the 'dice' to use for NobleNote matching…
    static ref NOBLE_DICE: usize = {
        let num_str = NOBILITYFILE.chooser.trim_start_matches('d');
        num_str.parse::<usize>().expect(format!("Invalid 'chooser' format in '{}'", NOBLENOTES_FILE).as_str())
    };

    // Load and parse noble land titles…
    static ref NOBLE_TITLE_PARTS: NobleTitleParts = {
        serde_jsonc::from_str(
            &fs::read_to_string(NOBLE_TITLE_PARTS_FILE)
                .expect(format!("No '{}' found?!", NOBLE_TITLE_PARTS_FILE).as_str())
        ).expect("JSON error")
    };
}

#[derive(Debug, Deserialize, Clone)]
struct NobilityFile {
    chooser: String,
    titles: Vec<NobleNote>
}

#[derive(Debug, Deserialize)]
struct NobleTitleParts {
    part1: Vec<String>,
    part2: Vec<String>,
    part3: Vec<String>
}

impl NobleTitleParts {
    /// Create a single noble special/land title.
    fn create_title(&self) -> String {
        let r = 1.d(self.part1.len());
        let p1 = &self.part1[r-1];
        let r = 1.d(10 + self.part2.len());
        if r <= 10 {
            return p1.clone();
        }
        let r2 = 1.d(self.part3.len());
        format!("{p1} of the {} {}", self.part2[r-11], self.part3[r2-1])
    }
}

/// A trait for anything that deals with **TiMod**.
pub trait TiMod {
    fn timod(&self) -> i32;
}

/// PC/NPC noble entry.
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Noble {
    name: (String, Option<String>),
    timod: usize,
    land_titles: Vec<String>,
    land_size: usize,
}

impl TiMod for Noble {
    fn timod(&self) -> i32 {
        self.timod.min(i32::MAX as usize) as i32
    }
}

impl IsNamed for Noble {
    fn name(&self) -> &str {
        &self.name.0
    }
}

impl From<&'static NobleNote> for Noble {
    fn from(value: &'static NobleNote) -> Self {
        Noble {
            name: value.name.clone(),
            timod: value.timod.random(),
            land_titles: (0..value.land_titles.random())
                .map(|_| NOBLE_TITLE_PARTS.create_title())
                .collect(),
            land_size: value.land_holdings.if_p(|| value.land_size.random()).unwrap_or_default(),
        }
    }
}

impl Noble {
    /// Generate a random culture-appropriate [Noble] entry.
    pub fn random(culture_core: &impl HasCultureCoreType) -> Self {
        let r = 1.d(*NOBLE_DICE);
        let c = culture_core.core_type().to_string();
        Self::from(NOBLENOTES.iter()
            .find(|n| n.culture.contains(&c) && n.roll_range().contains(&r))
            .expect(format!("No suitable NobleNote found for '{}' with roll of '{}'", c, r).as_str()))
    }

    /// See if the [Noble] entry is compatible with the given [Culture].
    pub fn is_compatible_with(&self, culture: &Culture) -> bool {
        NOBLENOTES.iter()
            .find(|n| n.name() == self.name() && n.culture.contains(&culture.core_type().to_string())).is_some()
    }
}

/// A simple (NPC) noble entry for simple purposes…
#[derive(Debug, Deserialize, Serialize, Clone, Gendered)]
pub struct SimpleNobleNPC {
    pub name: String,
    gender: Gender,
    pub nobility: Noble,
}

impl SimpleNobleNPC {
    /// Generate a random noble (NPC) with/from the given specs.
    pub fn new_cultured(name: &str, culture_core: &impl HasCultureCoreType) -> Self {
        Self {
            name: name.to_string(),
            gender: Gender::random_biased(GenderBias::None),
            nobility: Noble::random(culture_core)
        }
    }

    /// Alter gender to be `gender`.
    pub fn with_gender(mut self, gender: &Gender) -> Self {
        self.gender = gender.clone();
        self
    }

    /// Generate a random, named noble NPC.
    /// 
    /// FYI: [Culture] for them is random. For pre-defined [Culture], use `new_cultured()` instead.
    pub fn new(name: &str) -> Self {
        Self::new_cultured(name,
            match 1.d20() {
            ..=1 => CultureCoreType::Primitive,
            ..=5 => CultureCoreType::Nomad,
            ..=10 => CultureCoreType::Barbarian,
            ..=17 => CultureCoreType::Civilized,
            _ => CultureCoreType::Decadent
        }.core_type())
    }
}

#[derive(Debug, Deserialize, Clone)]
#[serde(untagged)]
enum TiModDecisionMethod {
    Flat(i32),
    Roll((i32, usize)),
    DeriveFromParent
}

impl Default for TiModDecisionMethod {
    fn default() -> Self {
        Self::DeriveFromParent
    }
}

impl TiModDecisionMethod {
    fn random(&self) -> usize {
        (match self {
            Self::Flat(x) => *x,
            Self::Roll((n,s)) => n.d(*s),
            Self::DeriveFromParent => 0
        }).max(0) as usize
    }
}

#[derive(Debug, Deserialize, Clone)]
struct ValueRange {
    range: std::ops::RangeInclusive<i32>
}

#[derive(Debug, Deserialize, Clone)]
struct PercentChance {
    p: u32
}

#[derive(Debug, Deserialize, Clone)]
#[serde(untagged)]
enum LandTitlesDecisionMethod {
    CountRange(ValueRange),
    FixedCount(i32),
    RollRange((i32,usize)),
    PercentageChanceOfOne(PercentChance),
    DeriveFromParent,
}

impl LandTitlesDecisionMethod {
    fn random(&self) -> usize {
        (match self {
            Self::CountRange(r) => r.range.random_of(),
            Self::FixedCount(x) => *x,
            Self::RollRange((n, s)) => n.d(*s),
            Self::PercentageChanceOfOne(p) => percentage_chance_of!(p.p, 1),
            Self::DeriveFromParent => 0
        }).max(0) as usize
    }
}

impl Default for LandTitlesDecisionMethod {
    fn default() -> Self {
        Self::DeriveFromParent
    }
}

#[derive(Debug, Deserialize, Clone)]
struct DiceMul {
    mul: i32
}

#[derive(Debug, Deserialize, Clone)]
#[serde(untagged)]
enum LandSizeDecisionMethod {
    NoModRoll((i32,usize)),
    ModRoll((i32,usize,i32)),
    MulRoll((i32,usize,DiceMul)),
    // "*" representation
    DeriveFromParent,
}

impl Default for LandSizeDecisionMethod {
    fn default() -> Self {
        Self::DeriveFromParent
    }
}

impl LandSizeDecisionMethod {
    fn random(&self) -> usize {
        (match self {
            Self::DeriveFromParent => 0,
            Self::ModRoll((n,s,m)) => n.d(*s) + m,
            Self::MulRoll((n,s,m)) => n.d(*s) * m.mul,
            Self::NoModRoll((n,s)) => n.d(*s),
        }).max(0) as usize
    }
}

#[derive(Debug, Deserialize, Clone, HasRollRange)]
struct NobleNote {
    #[serde(deserialize_with = "deserialize_string_w_optional")]
    name: (String, Option<String>),
    #[serde(default)] timod: TiModDecisionMethod,
    #[serde(default)] land_titles: LandTitlesDecisionMethod,
    #[serde(default)] land_holdings: usize,
    #[serde(default)] land_size: LandSizeDecisionMethod,
    #[serde(deserialize_with = "deserialize_strings_to_vec")]
    culture: Vec<String>,
    #[serde(deserialize_with = "deserialize_cr_range")]
    _cr_range: std::ops::RangeInclusive<i32>,
    #[serde(default)] derive_from_parent_if: Option<u32>,
}

impl IsNamed for NobleNote {
    fn name(&self) -> &str {
        &self.name.0
    }
}

impl NobleNote {
    /// Get a culture appropriate random noble status.
    fn random(culture_core: &impl HasCultureCoreType) -> &'static NobleNote {
        let core_name = culture_core.core_type().to_string();
        let roll = 1.d100();

        NOBLENOTES.iter()
            .filter(|note| note.culture.contains(&core_name))
            .find(|note| note.roll_range().contains(&roll))
            .expect("What on Earth just happened...?")
    }
}

#[cfg(test)]
mod nobility_data_integrity_tests {
    use super::*;

    static SPAM_COUNT: usize = 1001;

    #[test]
    fn nobility_json_ok() {
        assert!(NOBLENOTES.len() > 0);
    }

    #[test]
    fn noblenote_for_barbarian_spam() {
        let ct = CultureCoreType::Barbarian;
        for _ in 0..SPAM_COUNT {
            let _ = NobleNote::random(&ct);
        }
    }

    #[test]
    fn prince() {
        let _ = env_logger::try_init();
        let parent = NOBLENOTES.iter()
            .find(|c| c.name() == "Baron")
            .expect("Baron -should- exist!");
        let prince = NOBLENOTES.iter()
            .find(|c| c.name() == "Prince")
            .expect("Prince -should- exist!");
        // redo a few times…
        for _ in 0..20 {
            if let Some(archd_chance) = prince.derive_from_parent_if {
                if 1.d100() > archd_chance {
                    let land_holdings = parent.land_holdings.if_p(||parent.land_size.random());
                    log::debug!("Got {:?} as land_holdings.", land_holdings);
                } else {
                    log::debug!("ArchD, skip.");
                }
            }
        }
    }

    #[test]
    fn titles_are_accounted_for() {
        let all_expected = vec![
            "Emperor","High King","King","Kahn","Royal Prince","Archduke","Duke",
            "Marquis","Chieftain","Viscount","Jarl","Subchieftain","Count","Baron",
            "Baronet","Prince","Knight","Hetman"];
        let prim_expected = vec![
            "High King","Chieftain","Subchieftain"];
        let nomad_expected = vec![
            "Kahn","Chieftain","Subchieftain","Hetman"];
        let barb_expected = vec![
            "High King","King","Royal Prince","Chieftain","Jarl","Subchieftain","Baron","Prince","Hetman"];
        let civdec_expected = vec![
            "Emperor","King","Royal Prince","Archduke","Duke",
            "Marquis","Viscount","Count","Baron",
            "Baronet","Prince","Knight"];
        let cultures = [
            ("Primitive", &prim_expected),
            ("Nomad", &nomad_expected),
            ("Barbarian", &barb_expected),
            ("Civilized", &civdec_expected),
            ("Decadent", &civdec_expected)
        ];

        assert!(all_expected
            .iter()
            .all(|t|
                NOBLENOTES.iter()
                    .find(|n| n.name() == *t)
                    .is_some()
            ));

        cultures.iter().for_each(|(n,v)|{
            assert_eq!(NOBLENOTES.iter().filter(|note|note.culture.contains(&n.to_string())).collect::<Vec<&NobleNote>>().len(), v.len());
            assert!(NOBLENOTES.iter()
                .filter(|note|note.culture.contains(&n.to_string()))
                .all(|note| v.contains(&note.name())));
            assert!(v.iter()
                .all(|t| NOBLENOTES.iter()
                    .find(|note| note.name() == *t && note.culture.contains(&n.to_string()))
                    .is_some()));
        });
    }
}