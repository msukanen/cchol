//! Shapes
//! 
//! JSON partially derived from 866.
use std::{fmt::Display, fs};

use dicebag::{DiceExt, RandomOf};
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

use rpgassist::{resolve::resolve_in_place::ResolveInPlace, serialize::serial_strings::deserialize_strings_to_vec};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Shape {
    name: String,
    #[serde(default, deserialize_with = "deserialize_strings_to_vec")]
    alt: Vec<String>,
    /// `true|false` the shape is birthmarkable?
    #[serde(default)] bm: bool,
}

static SHAPE_FILE: &'static str = "./data/shape.json";
lazy_static! {
    static ref SHAPES: Vec<Shape> = {
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum ShapeEntry {
            S(String),
            C(Shape),
        }
        fn shaper(se: ShapeEntry) -> Shape {
            match se {
                ShapeEntry::S(name) => Shape { name, alt: vec![], bm: false },
                ShapeEntry::C(shape) => shape
            }
        }

        let json = &fs::read_to_string(SHAPE_FILE).expect(format!("Error with '{SHAPE_FILE}'?!").as_str());
        let items: Vec<ShapeEntry> = serde_jsonc::from_str(json).expect("JSON error");
        items.into_iter().map(shaper).collect()
    };
}

impl Display for Shape {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl Shape {
    pub fn random() -> Self {
        let mut base_shape = SHAPES.random_of();
        base_shape.resolve();
        base_shape
    }

    pub fn random_birthmarkable() -> Self {
        let oks: Vec<&Shape> = SHAPES.iter().filter(|s| s.bm == true).collect();
        let mut shape = oks.random_of().clone();
        shape.resolve();
        shape
    }
}

impl ResolveInPlace for Shape {
    fn resolve(&mut self) {
        if !self.alt.is_empty() {
            let roll = 1.d(self.alt.len()+1)-2;
            if roll >= 0 {
                self.name = self.alt[roll as usize].clone()
            }
        }
    }
}

#[cfg(test)]
mod shape_tests {
    use dicebag::RandomOf;

    use crate::misc::{Shape, shape::SHAPES};

    #[test]
    fn shape_file_data_integrity() {
        // raw check
        let _ = SHAPES.random_of();
        // resolve()'d check
        let _ = Shape::random();
    }
}