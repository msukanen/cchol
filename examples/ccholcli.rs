use cchol::player::PlayerCharacter;
use rpgassist::gender::Gender;

/// Simple CLI thingy.
fn main() {
    let p = PlayerCharacter::new("MJS", Some(Gender::Male), None);
}