use crate::ImplicitClone;
use std::fmt;

type Rc<T> = std::sync::Arc<T>;

include!("string.rs");
include!("array.rs");
#[cfg(feature = "map")]
include!("map.rs");

impl<T> ImplicitClone for Rc<T> {}
