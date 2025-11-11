//! 103: Social Status

use dicebag::DiceExt;
use rpgassist::resolve::resolve_in_place::ResolveInPlace;
use serde::{Deserialize, Serialize};

use crate::{modifier::SolMod, social::{nobility::Noble, wealth::Wealth}, traits::HasCulture};

/// Status specs.
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SocialStatus {
    pub(crate) wealth: Wealth,
    pub(crate) nobility: Option<Noble>,
} impl SocialStatus {
    /// Generate [`culture`][Culture]-appropriate random [SocialStatus].
    pub fn random(culture: &impl HasCulture) -> Self {
        let mut wealth = Wealth::random(culture).clone();
        wealth.resolve();
        let nobility = if 1.d100() + culture.cumod() >= 99 {
            Some(Noble::random(culture.core_type()))
        } else { None };
        Self { wealth, nobility }
    }

    /// Check whether current [SocialStatus] is compatible with the given [`culture`][Culture].
    pub fn is_compatible_with(&self, culture: &impl HasCulture) -> bool {
        let noble_compatible = if let Some(n) = &self.nobility {
            n.is_compatible_with(culture)
        } else { true };
        self.wealth.is_compatible_with(culture) && noble_compatible
    }

    /// Get reference to current [Wealth].
    pub fn wealth(&self) -> &Wealth {
        &self.wealth
    }

    /// Get reference to current [nobility][Noble], if any.
    pub fn nobility(&self) -> Option<&Noble> {
        self.nobility.as_ref()
    }

    /// Elevate to next higher nobility rung…
    pub fn elevate_nobility(&mut self, culture: &impl HasCulture) {
        if let Some(n) = &self.nobility {
            if let Some(next) = n.get_next_higher_rank(culture) {
                self.nobility = Some(next.into())
            }
        } else {
            self.nobility = Some(Noble::from(Noble::get_lowest_rank(culture)))
        }
    }

    /// There goes the noble titles, buh bye…
    pub fn demote_nobility(&mut self) {
        self.nobility = None
    }

    /// Set base starting money.
    /// 
    /// Character's actual starting money is dictated by [Wealth] from there on.
    pub fn set_base_starting_money(&mut self, amount: u32) {
        self.wealth.set_base_starting_money(amount)
    }

    pub fn starting_money(&self) -> f64 {
        self.wealth.starting_money()
    }
}

impl SolMod for SocialStatus {
    fn solmod(&self) -> i32 {
        // being a noble (born as such or otherwise) adds a flat '5' to SolMod value.
        self.wealth.solmod() + if let Some(_) = &self.nobility {5} else {0}
    }
}