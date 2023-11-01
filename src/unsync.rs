use std::rc::Rc;

use crate::ImplicitClone;

#[path = "array.rs"]
mod array;
#[cfg(feature = "map")]
#[path = "map.rs"]
mod map;
#[path = "string.rs"]
mod string;

pub use array::*;
#[cfg(feature = "map")]
pub use map::*;
pub use string::*;

impl<T: ?Sized> ImplicitClone for Rc<T> {}
