use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
pub enum Level {
    Primitive,
    Nomad,
    Barbarian,
    Civilized,
    Decadent
}
