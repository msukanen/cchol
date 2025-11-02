use dicebag::DiceExt;

fn main() {
    println!("Hello, world!");
}

type RollRange = std::ops::RangeInclusive<i32>;

trait HasRollRange {
    fn roll_range(&self) -> &RollRange;
}

trait RollInRollRange<T> {
    fn random(&self, range: &RollRange) -> &T;
}

impl <T> RollInRollRange<T> for Vec<T>
where
    T: HasRollRange,
    T: Clone,
{
    fn random(&self, range: &RollRange) -> &T {
        let roll = 1.d((*range.end()-*range.start()+1) as usize);
        self.iter()
            .find(|v| v.roll_range().contains(&roll))
            .expect("OMG")
    }
}