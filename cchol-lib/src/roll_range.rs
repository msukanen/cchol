use dicebag::InclusiveRandomRange;

pub type RollRange = std::ops::RangeInclusive<i32>;

pub trait HasRollRange {
    fn roll_range(&self) -> &RollRange;
}

pub(crate) trait HasVecRollRangeRoll<T>: HasRollRange {
    fn get_in_range(&self, range: &RollRange) -> &T;
    fn random_by_cr(&self, range: &RollRange) -> T;
}

impl <T> HasVecRollRangeRoll<T> for Vec<T>
where
    T: HasRollRange,
    T: Clone,
{
    fn get_in_range(&self, range: &RollRange) -> &T {
        let roll = range.random_of();
        self.iter()
            .find(|r| r.roll_range().contains(&roll))
            .expect(format!("Roll '{roll}' out of range or some other logic failure…").as_str())
    }

    fn random_by_cr(&self, range: &std::ops::RangeInclusive<i32>) -> T {
        self.get_within_cr(range).clone()
    }
}

impl <T> HasVecRollRangeRoll for &Vec<T>
where
    T: HasRollRange,
    T: Clone,
{
    fn get_in_range(&self, range: &RollRange) -> &T {
        (*self).get_in_range(range)
    }

    fn random_by_cr(&self, range: &RollRange) -> T {
        (*self).random_by_cr(range)
    }
}