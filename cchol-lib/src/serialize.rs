use serde::{Deserialize, Deserializer};

/// Deserializer for `_cr_range` field.
/// 
/// # `_cr_range` in JSON
/// * `i32` — singular choice value.
/// * `[i32, i32]` — inclusive range.
/// * `{ "upto": i32 }` — an inclusive "anything upto X" range's cap.
pub(crate) fn deserialize_cr_range<'de, D>(deserializer: D) -> Result<std::ops::RangeInclusive<i32>, D::Error>
where D: Deserializer<'de> {
    #[derive(Deserialize)]
    struct UptoX {
        upto: i32
    }
    // A helper for dual format input:
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum RangeHalp {
        S(i32),
        P((i32,i32)),
        U(UptoX)
    }

    match RangeHalp::deserialize(deserializer)? {
        RangeHalp::S(v) => Ok(v..=v),
        RangeHalp::P((a,b)) => Ok(a..=b),
        RangeHalp::U(x) => Ok(i32::MIN..=x.upto),
    }
}

/// Deserializer for optional `_cr_range` field.
/// 
/// # `_cr_range` in JSON
/// * `i32` — singular choice value.
/// * `[i32, i32]` — inclusive range.
/// * `{ "upto": i32 }` — an inclusive "anything upto X" range's cap.
pub(crate) fn deserialize_optional_cr_range<'de, D>(deserializer: D)
    -> Result<Option<std::ops::RangeInclusive<i32>>, D::Error>
where D: Deserializer<'de> {
    #[derive(Deserialize)]
    struct UptoX {
        upto: i32
    }
    // A helper for dual format input:
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum RangeHalp {
        S(i32),
        P((i32,i32)),
        U(UptoX)
    }

    let maybe_halp = Option::<RangeHalp>::deserialize(deserializer)?;

    match maybe_halp {
        Some(RangeHalp::S(v)) => Ok(Some(v..=v)),
        Some(RangeHalp::P((a,b))) => Ok(Some(a..=b)),
        Some(RangeHalp::U(x)) => Ok(Some(i32::MIN..=x.upto)),
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
        DiceHalp::S(v) => Ok((v, v)),
        DiceHalp::P((a,b)) => Ok((a, b))
    }
}