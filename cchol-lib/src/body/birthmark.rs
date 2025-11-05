//! 866: Birthmarks
use dicebag::DiceExt;
use rpgassist::{body::location::BodyLocation, misc::shape::Shape};
use serde::{Deserialize, Serialize};

use crate::misc::ExoticColor;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Birthmark {
    pub location: BodyLocation,
    pub exotic_color: Option<ExoticColor>,
    pub shape: Shape,
}

impl Birthmark {
    /// Generate a random birthmark with location; also color if color is other than "natural".
    // T-866
    pub fn new() -> Self {
        let location = BodyLocation::random();
        let exotic_color = if 1.d20() == 1 {Some(ExoticColor::random())} else {None};
        let shape = Shape::new();

        Self { location, exotic_color, shape }
    }
}