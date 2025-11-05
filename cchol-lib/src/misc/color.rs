//! 865: Color
//! 
//! Some colors are exotic in some contexts…
use std::fs;

use dicebag::{DiceExt, IsOne, RandomOf};
use lazy_static::lazy_static;
use rpgassist::resolve::resolve_in_place::ResolveInPlace;
use serde::{Deserialize, Deserializer, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum ColorTint {
    Pastel,
    Dark
} impl ColorTint {
    /// Randomly either [pastel/light][ColorTint::Pastel] or [dark][ColorTint::Dark].
    fn random() -> Self {
        if 1.d2().is_one() {Self::Pastel} else {Self::Dark}
    }
}

static TINT_1_IN_X_CHANCE_EXOTIC: usize = 6;   // 1:6 chance for exotic color to have additional tint applied.
static TINT_1_IN_X_CHANCE_MUNDANE: usize = 20; // 1:20 chance for a mundane color to have additional tint applied.

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ColorVariant {
    name: String,
    #[serde(default)]
    exotic: bool,
}

/// Deserializes the "alt" field, which can be
/// a) single string,
/// b) an array of strings, or
/// c) mixed array of strings and objects.
fn deserialize_color_variety<'de, D>(deserializer: D) -> Result<Vec<ColorVariant>, D::Error>
where D: Deserializer<'de>
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum AltItem {
        S(String),
        V(ColorVariant)
    }

    #[derive(Deserialize)]
    #[serde(untagged)]
    enum AltField {
        S(String),
        M(Vec<AltItem>),
    }

    impl From<AltItem> for ColorVariant {
        fn from(value: AltItem) -> Self {
            match value {
                AltItem::S(name) => ColorVariant { name, exotic: false },
                AltItem::V(v) => v,
            }
        }
    }

    match AltField::deserialize(deserializer)? {
        AltField::S(name) => Ok(vec![ColorVariant{name, exotic: false}]),
        AltField::M(items) => Ok(items.into_iter().map(ColorVariant::from).collect())
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ExoticColor {
    #[serde(default)]
    tint: Option<ColorTint>,
    name: String,
    #[serde(default, deserialize_with = "deserialize_color_variety")]
    alt: Vec<ColorVariant>,
    #[serde(default)]
    exotic: bool,
}

trait ColorValidator{fn validate_colors(self)->Vec<ExoticColor>;}
impl ColorValidator for Vec<ExoticColor> {
    fn validate_colors(self)->Vec<ExoticColor> {
        if self.iter()
            .filter(|c|!c.exotic)
            .collect::<Vec<&ExoticColor>>()
            .is_empty() {
                panic!("No mundane colors defined in '{COLOR_FILE}'!")
            }
        self
    }
}

static COLOR_FILE: &'static str = "./data/color.json";
lazy_static! {
    static ref COLORS: Vec<ExoticColor> = serde_jsonc::from_str::<Vec<ExoticColor>>(
        &fs::read_to_string(COLOR_FILE).expect(format!("Error with '{COLOR_FILE}'?!").as_str())
    )
    .expect("JSON error")
    .validate_colors();
}

impl ResolveInPlace for ExoticColor {
    /// Resolve actual color whether it's the base or alt variant, and tint of it (if any).
    fn resolve(&mut self) {
        if !self.alt.is_empty() {
            let roll = 1_i32.d(self.alt.len() + 1)-2;
            if roll >= 0 {
                let alt = &self.alt[roll as usize];
                self.name = alt.name.clone();
                self.exotic = alt.exotic;
            }
        }
        self.tint = if 1.d(TINT_1_IN_X_CHANCE_EXOTIC).is_one() {Some(ColorTint::random())} else {None};
    }
}

impl ExoticColor {
    /// Resolve actual color whether it's the base or alt variant, and tint of it (if any).
    /// Exotic colors and variants are not taken into account when selecting.
    fn resolve_as_mundane(&mut self) {
        if !self.alt.is_empty() {
            let munds: Vec<&str> = self.alt.iter().filter(|c| c.exotic == false).map(|c| c.name.as_str()).collect();
            if !munds.is_empty() {
                let roll = 1_i32.d(munds.len() + 1)-2;
                if roll >= 0 {
                    self.name = munds[roll as usize].to_string();
                }
            }
        }
        self.tint = if 1.d(TINT_1_IN_X_CHANCE_MUNDANE).is_one() {Some(ColorTint::random())} else {None};
    }

    /// Generate a random color, exotics (and tint, if any) included.
    pub fn random() -> Self {
        let mut base_color = COLORS.get(1.d(COLORS.len()) - 1).cloned().unwrap();
        base_color.resolve();
        base_color
    }

    /// Generate a random mundane color. Tint included, if any.
    pub fn random_mundane() -> Self {
        let mut base_color
            = COLORS.iter()
            .filter(|c| !c.exotic)
            .collect::<Vec<&ExoticColor>>()
            .random_of()
            .clone();
        base_color.resolve_as_mundane();
        base_color
    }
}

#[cfg(test)]
mod color_tests {
    use crate::misc::color::COLORS;

    #[test]
    fn color_file_data_integrity() {
        let _ = COLORS.first().cloned().unwrap();
        assert!(0 < COLORS.len());
    }

    #[test]
    fn filter_out_exotic_base_color() {
        if let Some(_) = COLORS.iter().filter(|c| c.exotic != true).find(|c| c.name.to_lowercase() == "pink") {
            panic!("Pink was not supposed to be found when filtering exotics away!")
        }
    }
}