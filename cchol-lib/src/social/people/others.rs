//! 750: Others
use serde::{Deserialize, Serialize};

use crate::social::people::govt_official::GovtOfficial;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum OtherPeople {
    GovtOfficial(GovtOfficial)
}