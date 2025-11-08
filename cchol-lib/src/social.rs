pub mod birth;
pub use birth::BiMod;
pub mod birth_legitimacy;
pub mod culture; pub use culture::CuMod;
pub mod family;
    mod lineage; pub use lineage::LineageStrictness;
pub mod nobility;
pub mod people;
    mod religion; pub use religion::Deity;
pub mod status;
pub mod wealth;

/// A trait for anything and everything that delivers **SolMod**.
pub trait SolMod {
    fn solmod(&self) -> i32;
}