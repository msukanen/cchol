//! 754: Guardians

use rpgassist::gender::{Gender, HasGender};
use serde::{Deserialize, Serialize};

use crate::social::{family::FamilyStructure, people::Relation};

/// Guardians, guardian-like, and others…
#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum Guardian {
    Relative(Relation),
    Orphanage,
    Adopted(Box<FamilyStructure>),
} impl HasGender for Guardian {
    fn gender(&self) -> Gender {
        match self {
            Self::Relative(r) => r.gender(),
            Self::Orphanage => Gender::NeverApplicable,
            Self::Adopted(f) => f.gender(),
        }
    }
}