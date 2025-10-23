use dicebag::DiceExt;
use serde::{Deserialize, Serialize};

/// Some time periods.
#[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq, PartialOrd)]
pub enum TimePeriod {
    Past,
    Present,
    Future,
}

impl TimePeriod {
    /// Generate a random time period.
    pub fn new() -> Self {
        match 1.d3() {
            ..=1 => Self::Past,
            2 => Self::Present,
            _ => Self::Future,
        }
    }

    /// Generate a random non-present time period. Bias is 75% toward the past.
    pub fn new_nonpresent() -> Self {
        if 1.d20() < 16 { Self::Past } else { Self::Future }
    }
}