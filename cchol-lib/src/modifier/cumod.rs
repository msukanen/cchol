use crate::society::culture::CultureLevelType;

pub trait CuMod {
    fn cumod(&self) -> i32;
    fn as_clt(&self) -> CultureLevelType {CultureLevelType::from(self.cumod())}
}