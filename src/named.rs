/// A trait for anything with 'name'.
pub trait IsNamed {
    fn name(&self) -> &str;
}