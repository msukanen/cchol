pub mod animal;
pub mod body;
pub mod events;
pub(crate) mod ext;
pub mod misc;
pub mod modifier;
pub mod pc;
pub mod places;
pub mod racial;
pub mod roll_range;
pub(crate) mod serialize;
pub mod skill;
pub mod social;
mod stat;
use rpgassist::gender::Gender;
pub use stat::StatMap;
mod traits;
pub(crate) mod string_manip;
mod workpad;
pub(crate) use workpad::Workpad;

use crate::{racial::Race, social::{birth::Birth, culture::Culture, status::SocialStatus}, traits::HasCulture};

pub fn generate_player_background(
    name: &str,
    gender: Option<&str>,
    race: Option<&str>,
    culture: Option<&str>,
) {
    let mut workpad = Workpad::new();
    workpad.set_name(name);
    workpad += Race::from(race);
    workpad += workpad.race().adjust_gender(Gender::from(gender));
    workpad += workpad.race().shift_culture_if_needed(Culture::from(culture));
    workpad += SocialStatus::random(workpad.culture());
    let birth = Birth::random(&mut workpad);
    workpad += birth;
}