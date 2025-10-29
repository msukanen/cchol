//! 103: Social Status

use serde::Deserialize;

use crate::social::wealth::Wealth;

/// Status specs.
#[derive(Debug, Deserialize, Clone)]
pub struct SocialStatus {
    wealth: Wealth,
    
}