
use crate::Block;

impl Block for String {
    type CreationInstructions = String;

    fn create(value: &String) -> Self { value.to_owned() }
    fn void() -> Self { String::default() }
    fn is_void(&self) -> bool { self.is_empty() }
}

macro_rules! impl_num {
    ($num:ty) => {
        impl Block for $num {
            type CreationInstructions = $num;

            fn create(value: &$num) -> Self { *value }
            fn void() -> Self { Self::default() }
            fn is_void(&self) -> bool { *self == Self::default() }
        }
    }
}

impl_num!(u8);
impl_num!(u16);
impl_num!(u32);
impl_num!(u64);
impl_num!(u128);

impl_num!(usize);

impl_num!(i8);
impl_num!(i16);
impl_num!(i32);
impl_num!(i64);
impl_num!(i128);

impl_num!(f32);
impl_num!(f64);

