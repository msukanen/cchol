//! 103: Social Status
use crate::{modifier::SolMod, society::{culture::CultureLevelType, nobility::Nobility, wealth::WealthLevel}};

pub struct SocialStatus {
    pub wealth: WealthLevel,
    pub nobility: Option<Nobility>,
}

impl SolMod for SocialStatus {
    fn solmod(&self) -> i32 {
        self.wealth.solmod() + if let Some(_) = &self.nobility {5} else {0}
    }
}

impl SocialStatus {
    pub fn new(culture_type: &CultureLevelType) -> Self {
        let make_noble = Nobility::is_eligible_r(culture_type);
    }
}