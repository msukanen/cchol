pub mod modifier;
pub mod racial;
pub(crate) mod serialize;
pub mod skill;
pub mod social;
mod traits;
pub use traits::IsNamed;

#[macro_export]
macro_rules! create_dice_size {
    ($table:ident) => {
        {
            $table.iter()
                .map(|e| *e.roll_range().end())
                .max()
                .unwrap_or_else(|| panic!("{} list is empty?!", stringify!($table)))
        } as usize
    };
}