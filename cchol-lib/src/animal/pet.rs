use std::collections::{HashMap, HashSet};

use dicebag::{DiceExt, lo, HiLo};
use rpgassist::{stat::Stat, serialize::serial_uf64::deserialize as uf64deserialize};
use serde::{Deserialize, Serialize};

use crate::misc::{ExoticColor, Substance};

/// PetAbility enum variant count.
///
/// This count is required for "ID-stacking" within `random()`.
static PAB_ENUM_COUNT: usize = 19;
/// Special pet abilities.
#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum PetAbility {
    /// If the species is winged by default, then they have an *extra* pair of wings with this.
    Wings { pairs: u8 },
    HighIQ { stat: Stat, can_speak: bool },
    Telepathic,
    UnusualColor { colors: Vec<ExoticColor> },
    UnusualSubstance { materials: HashSet<Substance> },
    //-- 5
    //TODO T874 - PhysicalAffliction(PhysicalAffliction)
    CanUseMagic,
    InvisibleToAllButOwner,
    Regenerates { speed_factor: u8 },
    PossessesNearestAnimalIfDies,
    //-- 10
    UnusualSize {
        #[serde(deserialize_with = "uf64deserialize")]
        diff_from_norm: f64
    },
    OncePerDayAssumeHumanoidForm { hour_duration_dice_size: u8 },
    RequiresManaToSurvive { mp_leech_per_day: u8 },
    ActsAsManaBattery { mp_reserve_per_day: u8 },
    AugmentsOwnerHP { hp_amount_added: u8 },
    //-- 15
    BreathesFire { dmg_mod: u8 },
    CanEnlargeSelf { factor_dice_size: u8, hour_duration_dice_size: u8 },
    CanProvideCoinDaily { gold_worth_dice_size: u8 },
    CanDiscorporateIntoMist,
} impl PetAbility {
    pub fn random() -> Vec<Self> {
        let mut pab_collection: HashMap<u8, u8> = HashMap::new();
        let mut i = 1;
        // Generate at most 4 distinct IDs.
        while i > 0 && pab_collection.len() < 4 {
            i -= 1;
            // Roll for key/ID - use number of PetAbility enum's entries+1 for dice size.
            let key = 1.d(PAB_ENUM_COUNT + 1);
            if key > PAB_ENUM_COUNT {
                i += 1.d3();
                continue;
            }
            // per-ID 'stack' count:
            *pab_collection.entry(key as u8).or_insert(0) += 1;
        }

        let mut pabs = vec![];
        // Construct actual PetAbility from each ID, stack count's effect varying per variant…
        pab_collection.iter().for_each(|(id, stack)|{
            pabs.push(match id {
                ..=1 => Self::Wings { pairs: *stack },
                2 => Self::HighIQ { stat: Stat::Int { val: 10 + *stack as i32 }, can_speak: 1.d100() <= 60 + *stack as usize * 10 },
                3 => Self::Telepathic,
                4 => Self::UnusualColor { colors: {
                    // why a vector? Color gradients! Or stripes, or something…
                    let mut cs = vec![];
                    for _ in 0..*stack {
                        cs.push(ExoticColor::random());
                    }
                    cs
                }},
                5 => Self::UnusualSubstance { materials: {
                    let mut mats = HashSet::new();
                    // sum up all the material types
                    for _ in 0..*stack {
                        mats.extend(Substance::random())
                    }
                    mats
                }},
                6 => unimplemented!("TODO 874"),
                7 => Self::CanUseMagic,
                8 => Self::InvisibleToAllButOwner,
                9 => Self::Regenerates { speed_factor: *stack },
                10 => Self::PossessesNearestAnimalIfDies,
                11 => Self::UnusualSize { diff_from_norm: if lo!() {1.0 / *stack as f64} else {1.0 * *stack as f64}},
                12 => Self::OncePerDayAssumeHumanoidForm { hour_duration_dice_size: 6 + (*stack - 1) * 2 },
                13 => Self::RequiresManaToSurvive { mp_leech_per_day: *stack },
                14 => Self::ActsAsManaBattery { mp_reserve_per_day: 1 << (*stack - 1) },
                15 => Self::AugmentsOwnerHP { hp_amount_added: 1 << (*stack - 1) },
                16 => Self::BreathesFire { dmg_mod: *stack - 1 },
                17 => Self::CanEnlargeSelf { factor_dice_size: 10 + (*stack * 2), hour_duration_dice_size: 5 + *stack },
                18 => Self::CanProvideCoinDaily { gold_worth_dice_size: 4 + (*stack * 2) },
                19 => Self::CanDiscorporateIntoMist,
                _ => unreachable!("ID past {PAB_ENUM_COUNT} should not have happened…")
            })
        });
        pabs
    }
}