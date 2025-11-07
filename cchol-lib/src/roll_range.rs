use dicebag::{InclusiveRandomRange, RandomOf};
use rpgassist::ext::GetTypeName;

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
        let found = self.iter()
            .filter(|r| r.roll_range().contains(&roll))
            .collect::<Vec<&T>>();
        if found.is_empty() {
            panic!("{} - roll '{roll}' out of range or some other logic failure…", self.type_name())
        }
        found.random_of()
    }
}