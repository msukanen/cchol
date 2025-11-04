//! 103: Social Status

use dicebag::DiceExt;
use rpgassist::resolve::resolve_in_place::ResolveInPlace;
use serde::{Deserialize, Serialize};

use crate::{modifier::CuMod, social::{culture::{Culture, HasCultureCoreType}, nobility::Noble, wealth::Wealth}};

/// Status specs.
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SocialStatus {
    wealth: Wealth,
    nobility: Option<Noble>,
} impl SocialStatus {
    pub fn random(culture: &Culture) -> Self {
        let mut wealth = Wealth::random(culture).clone();
        wealth.resolve();
        let nobility = if 1.d100() + culture.cumod() >= 99 {
            Some(Noble::random(culture.core_type()))
        } else { None };
        Self { wealth, nobility }
    }
}