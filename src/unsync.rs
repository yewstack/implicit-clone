use std::rc::Rc;

use crate::ImplicitClone;

#[path = "array.rs"]
mod array;
#[cfg(feature = "map")]
#[path = "map.rs"]
mod map;
#[path = "string.rs"]
mod string;

pub use array::IArray;
#[cfg(feature = "map")]
pub use map::IMap;
pub use string::IString;

impl<T: ?Sized> ImplicitClone for Rc<T> {}
