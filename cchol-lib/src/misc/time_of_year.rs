use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
pub enum TimeOfYear {
    Spring,
    Summer,
    Autumn,
    Winter
}