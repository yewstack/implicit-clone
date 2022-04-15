use crate::ImplicitClone;
use std::fmt;
use std::sync::Arc as Rc;

include!("string.rs");
include!("array.rs");
#[cfg(feature = "map")]
include!("map.rs");

impl<T> ImplicitClone for Rc<T> {}
