//! Anything and everything related to
//!   time of birth,
//!   place of birth,
//!   unusual birth circumstances,
//! etc.
/// A trait for anything that gives out **BiMod**.
pub trait BiMod {
    /// Get **BiMod** (birth modifier).
    fn bimod(&self) -> i32;
}