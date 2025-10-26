pub trait HasRollRange {
    fn roll_range(&self) -> &std::ops::RangeInclusive<i32>;
}