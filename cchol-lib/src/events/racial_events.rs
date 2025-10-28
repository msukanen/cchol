use serde::Deserialize;

/// Some [Race][crate::racial::race::Race] specific event types.
#[derive(Debug, Deserialize, Clone, PartialEq)]
pub enum RacialEvent {
    Dwarf,
    Elf,
    Halfling,
    Monster
}