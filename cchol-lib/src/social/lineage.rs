use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
pub enum LineageStrictness {
    /// Paternity and bloodline purity are socially and legally paramount.
    /// (e.g., Humans, Dwarves in a feudal context). LegitMod applies.
    StrictPatrilineal,

    /// Family is tracked through the mother, or the entire community is the parent.
    /// Paternity is irrelevant for social status. (e.g., Fauns, some Nomads). LegitMod is ignored.
    CommunalOrMatriarchal,

    /// The concept of lineage and blood ties is irrelevant (e.g., Elementals, Golems, some Monsters).
    NotApplicable,

    /// Default state for races where rule hasn't been explicit specified.
    #[serde(other)]
    Unspecified
}

impl Default for LineageStrictness {
    fn default() -> Self {
        Self::StrictPatrilineal
    }
}