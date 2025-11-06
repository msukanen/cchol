//! Shapes
//! 
//! JSON partially derived from 866.
use std::fmt::Display;

use serde::{Deserialize, Serialize};

use rpgassist::serialize::serial_strings::deserialize_strings_to_vec;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Shape {
    name: String,
    #[serde(deserialize_with = "deserialize_strings_to_vec")]
    alt: Vec<String>,
    /// `true|false` the shape is birthmarkable?
    #[serde(default)] bm: bool,
}

impl Display for Shape {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}