mod color;
pub use color::ExoticColor;
pub mod defaults;
mod occupation;
pub use occupation::OccupationPerformance;
mod serious_wound;
pub use serious_wound::SeriousWound;
pub mod datum;
mod shape;
pub use shape::Shape;
mod material;
pub use material::Substance;

use dicebag::DiceExt;

pub trait ConditionalExec {
    fn if_p<F, T>(&self, f: F) -> Option<T>
        where F: FnOnce() -> T;
}

impl ConditionalExec for i32 {
    fn if_p<F, T>(&self, f: F) -> Option<T>
            where F: FnOnce() -> T {
        if *self == 0 { return None }
        if 1u32.d100() <= (*self as u32) {
            Some(f())
        } else {
            None
        }
    }
}

impl ConditionalExec for usize {
    fn if_p<F, T>(&self, f: F) -> Option<T>
            where F: FnOnce() -> T {
        if *self == 0 { return None }
        if 1_usize.d100() <= (*self as usize) {
            Some(f())
        } else {
            None
        }
    }
}

/// A range that is nigh impossible to roll with dice. Used for stuff that needs _cr_range to be present but are not in basic roll tables.
pub static NO_RANGE: std::ops::RangeInclusive<i32> = i32::MIN..=i32::MIN;
