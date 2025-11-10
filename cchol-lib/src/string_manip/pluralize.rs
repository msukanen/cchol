//! Pluralizing strings, gendered variants and otherwise.

use rpgassist::gender::Gender;

pub(crate) trait Pluralizer {
    fn pluralize(&self) -> String;
}

impl Pluralizer for &str {
    fn pluralize(&self) -> String {
        pluralize(self)
    }
}

/// Simple non-gendered pluralizer.
pub fn pluralize(word: &str) -> String {
    let lower = word.to_lowercase();
    if lower.ends_with("f") {
        match lower.as_str() {
        //  "dwarf" |
        //  "elf"   |
            "wharf" => return format!("{word}s"),
            _       => return format!("{}ves", &word[..word.len()-1])
        }
    }

    format!("{word}s")
}

/// Gendered pluralizer.
pub fn pluralize_gendered(word: &str, gender: &Gender) -> String {
    let lower = word.to_lowercase();

    match lower.as_str() {
        // gender
        "man"   => return "men".into(),
        "woman" => return "women".into(),
        // gender specific pluralized word swaps
        "warlock" => return (if *gender == Gender::Female {"witches"} else {"warlocks"}).into(),
        "witch"   => return (if *gender == Gender::Female {"witches"} else {"warlocks"}).into(),
        // TODO more such words …
        _ => pluralize(word)
    }
}