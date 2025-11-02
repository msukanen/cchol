use dicebag::InclusiveRandomRange;

pub type RollRange = std::ops::RangeInclusive<i32>;

pub trait UseRollRange {
    fn roll_range(&self) -> &RollRange;
}

pub(crate) trait RollInRollRange<T> {
    fn get_random_in_range(&self, range: &RollRange) -> &T;
}

impl <T> RollInRollRange<T> for Vec<T>
where
    T: UseRollRange,
    T: Clone,
{
    fn get_random_in_range(&self, range: &RollRange) -> &T {
        let roll = range.random_of();
        self.iter()
            .find(|r| r.roll_range().contains(&roll))
            .expect(format!("Roll '{roll}' out of range or some other logic failure…").as_str())
    }
}