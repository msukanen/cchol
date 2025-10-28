//! Literacy is a skill among myriad others…

/// If something affects literacy chance, here's your chance to trait it so…
pub trait LitMod {
    /// Get literacy chance modifier (±%).
    fn litmod(&self) -> i32;
}

pub trait IsLiteracySource {
    /// Get potential literacy chances and languages.
    fn literacy_skills(&self) -> Vec<(String, i32)>;
}