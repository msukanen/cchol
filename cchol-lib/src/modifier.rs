//! A variety of modifiers.
//! 
//! # `BiMod`
//! 
//! [Birth]-based modifier.
//! 
//! # `CuMod`
//! 
//! [Culture]-based modifier.
//! 
//! # `LegitMod`
//! 
//! [IllegitimacyReason]- (and thus [Birth]-)based modifier.
//! 
//! # `SolMod`
//! 
//! [SocialStatus]-based modifier. To some degree affected by [LegitMod] and [Noble].
//! 
//! # `SurvivalMod`
//! 
//! Modifier to survival [skills][Skill].
//! Derived from a bunch of different places…
//! 
//! # `TiMod`
//! 
//! [Noble]-based modifier.
//! 
/// A trait for anything that gives out **BiMod**.
pub trait BiMod {
    /// Get the effective **BiMod**.
    fn bimod(&self) -> i32;
}

/// A trait for anything that acts (or routes) a **CuMod**.
pub trait CuMod {
    /// Get the effective **CuMod**.
    fn cumod(&self) -> i32;
}

/// A trait for anything that delivers birth related **LegitMod**.
pub trait LegitMod {
    /// Get the effective **LegitMod**.
    fn legitmod(&self) -> i32;
}

/// A trait for anything and everything that delivers **SolMod**.
pub trait SolMod {
    /// Get the effective **SolMod**.
    fn solmod(&self) -> i32;
}

/// A trait for anything that gives out some sort of a survival modifier.
pub trait SurvivalMod {
    /// Get associated survival mod.
    /// 
    /// # Returns
    /// 
    /// Zero if not otherwise specified.
    fn survival_mod(&self) -> i32 {0}
}

/// A trait for anything that deals with **TiMod**.
pub trait TiMod {
    /// Get the effective **TiMod**.
    fn timod(&self) -> i32;
}