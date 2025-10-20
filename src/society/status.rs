//! 103: Social Status
use crate::{modifier::{SolMod, SurvivalMod, TiMod}, society::{culture::CultureLevelType, nobility::Nobility, wealth::WealthLevel}};

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
    /// Generate random, culture appropriate, social status.
    pub fn new(culture_type: &CultureLevelType) -> Self {
        let nobility = if Nobility::is_eligible_r(culture_type) {Some(Nobility::new(culture_type))} else {None};
        let wealth = WealthLevel::new(culture_type, nobility.as_ref());

        Self { wealth, nobility }
    }
}

impl TiMod for SocialStatus {
    fn timod(&self) -> i32 {
        if let Some(n) = &self.nobility {
            n.timod()
        } else {0}
    }
}

impl SurvivalMod for SocialStatus {
    fn survmod(&self) -> i32 {
        self.wealth.survmod() + if let Some(_) = &self.nobility {-1} else {0}
    }
}