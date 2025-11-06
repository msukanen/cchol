//! 866: Birthmarks
use std::fmt::Display;

use dicebag::DiceExt;
use rpgassist::body::location::BodyLocation;
use serde::{Deserialize, Serialize};

use crate::misc::{ExoticColor, Shape};

/// Birthmark specs live here.
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Birthmark {
    location: BodyLocation,
    #[serde(default)]
    exotic_color: Option<ExoticColor>,
    shape: Shape,
}

impl Birthmark {
    /// Generate a random birthmark with location; also color if color is other than "natural".
    pub fn random() -> Self {
        let location = BodyLocation::random();
        let exotic_color = if 1.d20() == 1 {Some(ExoticColor::random())} else {None};
        let shape = Shape::random();

        Self { location, exotic_color, shape }
    }

    /// Relocate the birthmark. Generally of not much of use, but who knows…
    pub fn relocate(&mut self, where_to: BodyLocation) {
        self.location = where_to
    }
}

impl Display for Birthmark {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.exotic_color {
            None => write!(f, "{} as birthmark on {}", self.shape, self.location),
            Some(x) => write!(f, "{} {} as birthmark on {}", x, self.shape, self.location)
        }
    }
}