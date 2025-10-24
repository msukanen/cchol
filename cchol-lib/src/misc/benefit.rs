use serde::{Deserialize, Serialize};

/// A trait for anything that has benefits.
pub trait HasBenefits {
    /// Get (a to-be-parsed) vector of benefit effects.
    fn benefits(&self) -> Vec<Benefit>;
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum BenefitType {
    Skill,
    Stat,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(untagged)]
pub enum BenefitValue {
    Int(i32),
    r#String(String),
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Benefit {
    benefit_type: BenefitType,
    value: BenefitValue,
    is_for: String,
}

impl From<(BenefitType, String, String)> for Benefit {
    fn from(value: (BenefitType, String, String)) -> Self {
        Self {
            benefit_type: value.0,
            value: BenefitValue::r#String(value.1),
            is_for: value.2
        }
    }
}

impl From<(BenefitType, i32, String)> for Benefit {
    fn from(value: (BenefitType, i32, String)) -> Self {
        Self {
            benefit_type: value.0,
            value: BenefitValue::Int(value.1),
            is_for: value.2
        }
    }
}

impl From<(BenefitType, i32, &str)> for Benefit {
    fn from(value: (BenefitType, i32, &str)) -> Self {
        Self {
            benefit_type: value.0,
            value: BenefitValue::Int(value.1),
            is_for: value.2.to_string()
        }
    }
}