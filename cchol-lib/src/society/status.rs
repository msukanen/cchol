//! 103: Social Status
use serde::{Deserialize, Serialize};

use crate::{modifier::{CuMod, SolMod, SurvivalMod, TiMod}, society::{nobility::Nobility, wealth::WealthLevel}};

#[derive(Debug, Deserialize, Serialize)]
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
    /// 
    /// # Args
    /// `cumod_src`— some [CuMod] source.
    pub fn new(cumod_src: &impl CuMod) -> Self {
        let nobility = if Nobility::is_eligible_r(cumod_src) {Some(Nobility::new(cumod_src))} else {None};
        let wealth = WealthLevel::new(cumod_src, nobility.as_ref());

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