/// A trait for anything with "description" of some sort.
pub trait HasDescription {
    /// Get an immutable description of something.
    fn description(&self) -> &str;
    /// Get mutable reference to the description of something.
    fn description_mut(&mut self) -> &mut String;
}

/// A trait for anything which has an "explanation" beyond basic `.to_string()`.
pub trait IsExplained {
    /// Explain something in deeper-than-`to_string` detail.
    /// 
    /// Typical use case: explain footnotes et al.
    fn explain(&self) -> String;
}