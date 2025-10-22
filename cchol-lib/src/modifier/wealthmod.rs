//! WealthMod — wealth modifier.
pub trait WealthMod {
    fn wmod(&self) -> f64;
}