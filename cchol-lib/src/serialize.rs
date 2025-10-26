use serde::{Deserialize, Deserializer};

pub(crate) fn deserialize_cr_range<'de, D>(deserializer: D) -> Result<std::ops::RangeInclusive<i32>, D::Error>
where D: Deserializer<'de> {
    // A helper for dual format input:
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum RangeHalp {
        S(i32),
        P((i32,i32))
    }

    match RangeHalp::deserialize(deserializer)? {
        RangeHalp::S(v) => Ok(v..=v),
        RangeHalp::P((a,b)) => Ok(a..=b)
    }
}