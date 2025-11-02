pub mod culture;
pub mod nobility;
pub mod people;
pub mod status;
pub mod wealth;

/// A trait for anything and everything that delivers **SolMod**.
pub trait SolMod {
    fn solmod(&self) -> i32;
}