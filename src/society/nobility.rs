//! 758: Nobility

use serde::{Deserialize, Serialize};

use crate::gender::Gender;

/// A struct to haul around a barebones Noble NPC.
#[derive(Debug)]
pub struct NobleNPC {
    gender: Gender,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, PartialOrd)]
pub enum Title {
    
}
