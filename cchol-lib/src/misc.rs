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
