use serde::Deserialize;

/// Some [Race][crate::racial::race::Race] specific event types.
#[derive(Debug, Deserialize, Clone)]
pub enum RacialEvents {
    Dwarf,
    Elf,
    Halfling,
    Monster
}