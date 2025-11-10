use lazy_static::lazy_static;
use regex::Regex;
use rpgassist::ext::IsNamed;

use crate::{racial::{Monster, Race}, social::{Deity, culture::Culture, nobility::Noble}, string_manip::pluralize::Pluralizer};

lazy_static! {
    static ref RX_TAG_NAME: Regex = Regex::new(r"<(\w+)>").expect("Regex made a booboo with '<X>' for some reason!");
    static ref RX_MONSTER_NAME: Regex = Regex::new(r"<Monster>").expect("Regex made a booboo with '<Monster>' for some reason!");
    static ref RX_TAG_WITH_RANGE: Regex = Regex::new(r"<(\w+):(\w+)\.\.(\w+)>").expect("Regex made a booboo with '<X:Y..Z>' for some reason!");
    static ref RX_TAG_WITH_SPECIFIER: Regex = Regex::new(r"<(\w+):(\w+)>").expect("Regex made a booboo with '<X:Y>' for some reason!");
}

/// Resolve a variety of "hooks" into contained names.
pub(crate) fn resolve_name_hooks(candidate_name: &str, culture: &Culture) -> String {
    let mut resolved = candidate_name.to_string();
    
    // <Foobar> tags first…
    resolved = RX_TAG_NAME.replace_all(&resolved, |caps:&regex::Captures|{
        let tag = caps.get(1).unwrap().as_str();
        match tag {
            "Deity" => Deity::random(culture).name().to_string(),
            "Monster" => Monster::random().name().to_string(),
            "Nonhuman" => Race::random_nonhuman().name().to_string(),
            _ => unimplemented!("Unspecified tag '{tag}' encountered!")
        }
    }).into_owned();

    // Generic range-tag next…
    resolved = RX_TAG_WITH_RANGE.replace_all(&resolved, |caps: &regex::Captures|{
        let tag = caps.get(1).unwrap().as_str();
        let scope_start = caps.get(2).unwrap().as_str();
        let scope_end = caps.get(3).unwrap().as_str();

        match tag {
            "Noble" => {
                if let Some(note) = Noble::get_random_title_inclusive_between(scope_start, scope_end, culture) {
                    note.name().into()
                } else {
                    log::warn!("No suitable NobleNote found in '{scope_start}'..'{scope_end}' for '{}'", culture.name());
                    // range failure — fall back to original name …
                    resolved.clone()
                }
            },
            unknown => unimplemented!("`Tag-w/Range` encountered a tag ('{unknown}') with which we have no clue what to do with…")
        }
    }).into_owned();

    resolved = RX_TAG_WITH_SPECIFIER.replace_all(&resolved, |caps: &regex::Captures|{
        let tag = caps.get(1).unwrap().as_str();
        let specifier = caps.get(2).unwrap().as_str();
        
        // Check specific specifier(s) first as they dictate
        // what 'tag' is allowed to be used with them.
        match specifier {
            "plural" => match tag {
                "Nonhuman" => return Race::random_nonhuman().name().pluralize(),
                _ => unimplemented!("I don't know how to fetch or pluralize variant(s) of '{tag}'")
            },
            _ => {/* fall through */}
        }

        // Check tags; meaning of 'specifier' varies tag-by-tag…
        match tag {
            // For nobles, the 'specifier' means rank/title.
            "Noble" => {
                if let Some(note) = Noble::get_title_for_culture(specifier, culture) {
                    note.name().into()
                } else {
                    log::warn!("'{specifier}' was not a valid entry for '{}'", culture.name());
                    resolved.clone()
                }
            },
            _ => unimplemented!("No '{tag}':'{specifier}' combo implemented…")
        }
    }).into_owned();

    resolved
}