pub trait IsZero {
    fn is_zero(&self) -> bool;
}

impl IsZero for i32 {
    fn is_zero(&self) -> bool {
        *self <= 0
    }
}