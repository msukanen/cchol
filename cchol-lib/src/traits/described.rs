/// A trait for anything with "description" of some sort.
pub trait HasDescription {
    fn description(&self) -> &str;
    fn description_mut(&mut self) -> &mut String;
}