/// A trait for anything that needs rolls within a range.
pub trait HasRollRange {
    /// Get roll range.
    fn roll_range(&self) -> &std::ops::RangeInclusive<i32>;
}

/// Unified default impl of [HasRollRange] for anything with a suitable `_cr_range` field.
#[macro_export]
macro_rules! default_roll_range_def {
    ($for:ident) => {
        impl HasRollRange for $for {
            fn roll_range(&self) -> &std::ops::RangeInclusive<i32> { &self._cr_range }
        }
    };
}