use crate::ImplicitClone;
use std::fmt;

type Rc<T> = std::sync::Arc<T>;

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum ISlice<T: 'static + ?Sized> {
    /// A static string slice.
    Static(&'static T),
    /// A reference counted string slice.
    Rc(Rc<T>),
}

impl<T: 'static + ?Sized> Clone for ISlice<T> {
    fn clone(&self) -> Self {
        match self {
            Self::Static(x) => Self::Static(x),
            Self::Rc(x) => Self::Rc(x.clone()),
        }
    }
}

impl<T: 'static + ?Sized> ImplicitClone for ISlice<T> {}

include!("string.rs");
include!("array.rs");
#[cfg(feature = "map")]
include!("map.rs");

impl<T: ?Sized> ImplicitClone for Rc<T> {}
