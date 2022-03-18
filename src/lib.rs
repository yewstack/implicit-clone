mod array;
#[cfg(feature = "map")]
mod map;
mod string;

pub use array::*;
#[cfg(feature = "map")]
pub use map::*;
use std::rc::Rc;
pub use string::*;

pub trait ImplicitClone: Clone {}

impl<T: ImplicitClone> ImplicitClone for Option<T> {}
impl<T> ImplicitClone for Rc<T> {}

macro_rules! impl_implicit_clone {
    ($($ty:ty),+ $(,)?) => {
        $(impl ImplicitClone for $ty {})*
    };
}

#[rustfmt::skip]
impl_implicit_clone!(
    u8, u16, u32, u64, u128,
    i8, i16, i32, i64, i128,
    f32, f64,
    &'static str,
);
