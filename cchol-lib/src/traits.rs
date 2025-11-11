use rpgassist::ext::IsNamed;

use crate::{Workpad, modifier::CuMod, social::culture::{Culture, CultureCoreType}};

pub mod personality;

/// A trait for anything that returns a [Culture] reference.
/// 
/// **NOTE** Anything that delivers [Culture] has to also deliver
///          [CuMod] and [HasCultureCoreType].
///
pub trait HasCulture : CuMod + HasCultureCoreType + IsNamed {
    /// Get the underlying static [Culture] reference.
    fn culture(&self) -> &'static Culture;
}

/// A trait for anything that delivers [CultureCoreType]
pub trait HasCultureCoreType {
    /// Get the underlying [CultureCoreType].
    fn core_type(&self) -> &'static CultureCoreType;
}

/// Apply self directly onto the given [Workpad].
pub trait ApplyOnWorkpad {
    fn apply(&self, workpad: &mut Workpad) -> Result<(), String>;
}