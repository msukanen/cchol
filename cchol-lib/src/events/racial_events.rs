use serde::{Deserialize, Serialize};

/// Some [Race][crate::racial::race::Race] specific event types.
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub enum RacialEvent {
    Dwarf,
    Elf,
    Halfling,
    Monster
}