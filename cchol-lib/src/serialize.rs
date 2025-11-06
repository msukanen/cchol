//! Various serde deserializers:
//! 
//! * deserialize_cr_range — for potentially open-ended `_cr_range` fields.
//! * deserialize_fixed_cr_range — for fixed value `_cr_range` which have distinct start/end values in total.
//! * deserialize_optional_cr_range — as per `deserialize_cr_range`, but for Option variant.
//! * deserialize_dicet — deserializer for dice roll type(s).
//! * deserialize_strings_to_vec — deserialize a) single string, b) array of strings, both into vec of String.
//! 
//! Other:
//! * default_pc_save_cr_range — PC saves don't need `_cr_range` for anything, but it's present in some cases…
//! * validate_cr_ranges — validator for those `_cr_range` which need to be without gaps or overlaps.
use std::ops::RangeInclusive;

use serde::{Deserialize, Deserializer};

use crate::roll_range::UseRollRange;

/// Deserializer for `_cr_range` field.
/// 
/// # `_cr_range` in JSON
/// * `i32` — singular choice value.
/// * `[i32, i32]` — inclusive range.
/// * `{ "upto": i32 }` — an inclusive "anything upto X" range's cap.
/// * `{ "ge": i32 }` — anything greater-or-equal…
pub(crate) fn deserialize_cr_range<'de, D>(deserializer: D) -> Result<std::ops::RangeInclusive<i32>, D::Error>
where D: Deserializer<'de> {
    #[derive(Deserialize)]
    struct UptoX {
        upto: i32
    }
    #[derive(Deserialize)]
    struct GtOrEq {
        ge: i32
    }
    // A helper for dual format input:
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum RangeHalp {
        S(i32),
        P((i32,i32)),
        U(UptoX),
        G(GtOrEq),
    }

    match RangeHalp::deserialize(deserializer)? {
        RangeHalp::S(v) => Ok(v..=v),
        RangeHalp::P((a,b)) => Ok(a..=b),
        RangeHalp::U(x) => Ok(i32::MIN..=x.upto),
        RangeHalp::G(x) => Ok(x.ge..=i32::MAX),
    }
}

/// Deserializer for fixed-end `_cr_range`.
/// 
/// # `_cr_range` in JSON
/// * `i32` — singular choice value.
/// * `[i32, i32]` — inclusive range.
pub(crate) fn deserialize_fixed_cr_range<'de, D>(deserializer: D) -> Result<std::ops::RangeInclusive<i32>, D::Error>
where D: Deserializer<'de> {
    // A helper for dual format input:
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum RangeHalp {
        S(i32),
        P((i32,i32)),
    }

    match RangeHalp::deserialize(deserializer)? {
        RangeHalp::S(v) => Ok(v..=v),
        RangeHalp::P((a,b)) => Ok(a..=b),
    }
}

/// Deserializer for optional `_cr_range` field.
/// 
/// # `_cr_range` in JSON
/// * `i32` — singular choice value.
/// * `[i32, i32]` — inclusive range.
/// * `{ "upto": i32 }` — an inclusive "anything upto X" range's cap.
/// * `{ "ge": i32 }` — anything greater-or-equal…
pub(crate) fn deserialize_optional_cr_range<'de, D>(deserializer: D)
    -> Result<Option<std::ops::RangeInclusive<i32>>, D::Error>
where D: Deserializer<'de> {
    #[derive(Deserialize)]
    struct UptoX {
        upto: i32
    }
    #[derive(Deserialize)]
    struct GtOrEq {
        ge: i32
    }
    // A helper for dual format input:
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum RangeHalp {
        S(i32),
        P((i32,i32)),
        U(UptoX),
        G(GtOrEq),
    }

    let maybe_halp = Option::<RangeHalp>::deserialize(deserializer)?;

    match maybe_halp {
        Some(RangeHalp::S(v)) => Ok(Some(v..=v)),
        Some(RangeHalp::P((a,b))) => Ok(Some(a..=b)),
        Some(RangeHalp::U(x)) => Ok(Some(i32::MIN..=x.upto)),
        Some(RangeHalp::G(x)) => Ok(Some(x.ge..=i32::MAX)),
        None => Ok(None)
    }
}

/// Deserializer for DiceT/flat-i32.
/// 
/// # JSON format
/// * `i32`— a flat value.
/// * `[i32, i32]`— number of dice, number of dice sides.
pub(crate) fn deserialize_dicet<'de, D>(deserializer: D) -> Result<dicebag::DiceT, D::Error>
where D: Deserializer<'de> {
    // helper for dual format input
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum DiceHalp {
        S(i32),
        P((i32, i32))
    }

    match DiceHalp::deserialize(deserializer)? {
        DiceHalp::S(v) => Ok((v, 1)),
        DiceHalp::P((a,b)) => Ok((a, b))
    }
}

/// Deserializes the "culture" field, which can be a single String or a Vec<String>.
pub(crate) fn deserialize_string_w_optional<'de, D>(deserializer: D) -> Result<(String, Option<String>), D::Error>
where D: Deserializer<'de> {
    // helper for the two shapes
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum StringHalp {
        S(String),
        M((String, Option<String>)),
    }

    match StringHalp::deserialize(deserializer)? {
        StringHalp::S(s) => Ok((s, None)),
        StringHalp::M(v) => Ok(v),
    }
}

/// Dummy default value for PC save loading.
pub(crate) fn default_pc_save_cr_range() -> std::ops::RangeInclusive<i32> { 0..=0 }

/// Validate _cr_range entries to not have gaps/overlaps.
/// 
/// # Args
/// `vecname`— name/nick/whatever of that what has the `_cr_range` field.
/// `cr_source`— the `_cr_range` holding vector.
/// `opt_range_min`— default for range start is `1`, but can be adjusted with this.
/// 
/// # Return
/// Determined full-cover range.
pub(crate) fn validate_cr_ranges(
        vecname: &str,
        cr_source: &Vec<impl UseRollRange>,
        opt_range_min: Option<i32>
) -> std::ops::RangeInclusive<i32> {
    let mut ranges: Vec<&RangeInclusive<i32>> = cr_source
        .iter()
        .map(|c| c.roll_range())
        .collect();
    
    ranges.sort_by(|a,b| a.start().cmp(b.start()));
    
    if ranges.is_empty() {
        panic!("DATA VALIDATION: {vecname} list is empty. Cannot validate ranges.");
    }

    let min = opt_range_min.unwrap_or_else(||1);
    let start = *ranges[0].start();

    if start != min {
        panic!("DATA VALIDATION: {vecname} roll table must start at {min}. Found {:#?}", ranges[0]);
    }

    // Check for gaps/overlaps
    for w in ranges.windows(2) {
        let c = w[0];
        let n = w[1];
        let expected_next_start = *c.end() + 1;
        if *n.start() != expected_next_start {
            panic!("DATA VALIDATION: Gap or overlap in {vecname} roll table!\nFound {:#?}, followed by {:#?}", c, n);
        }
    }

    let end = ranges.last().unwrap().end();

    log::debug!("{vecname} ranges successfully validated: 1..={}", *ranges.last().unwrap().end());

    start..=*end
}