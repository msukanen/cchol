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
    /// Generate [`culture`][Culture]-appropriate random [SocialStatus].
    pub fn random(culture: &Culture) -> Self {
        let mut wealth = Wealth::random(culture).clone();
        wealth.resolve();
        let nobility = if 1.d100() + culture.cumod() >= 99 {
            Some(Noble::random(culture.core_type()))
        } else { None };
        Self { wealth, nobility }
    }

    /// Check whether current [SocialStatus] is compatible with the given [`culture`][Culture].
    pub fn is_compatible_with(&self, culture: &Culture) -> bool {
        let noble_compatible = if let Some(n) = &self.nobility {
            n.is_compatible_with(culture)
        } else { true };
        self.wealth.is_compatible_with(culture) && noble_compatible
    }
}