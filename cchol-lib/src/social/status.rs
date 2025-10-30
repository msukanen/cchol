//! 103: Social Status

use serde::{Deserialize, Serialize};

use crate::social::{nobility::Noble, wealth::Wealth};

/// Status specs.
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SocialStatus {
    wealth: Wealth,
    nobility: Option<Noble>,
}