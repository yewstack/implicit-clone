use std::rc::Rc;
use yew::html::{ImplicitClone, IntoPropValue};

#[derive(PartialEq)]
pub enum IArray<T: 'static> {
    Static(&'static [T]),
    Rc(Rc<[T]>),
}

impl<T: 'static> Clone for IArray<T> {
    fn clone(&self) -> Self {
        match self {
            Self::Static(a) => Self::Static(a),
            Self::Rc(a) => Self::Rc(a.clone()),
        }
    }
}

impl<T: 'static> Default for IArray<T> {
    fn default() -> Self {
        Self::Static(&[])
    }
}

impl<T: 'static> FromIterator<T> for IArray<T> {
    fn from_iter<I: IntoIterator<Item = T>>(it: I) -> Self {
        let vec = it.into_iter().collect::<Vec<T>>();
        Self::Rc(Rc::from(vec))
    }
}

impl<T: 'static> IntoPropValue<IArray<T>> for &'static [T] {
    fn into_prop_value(self) -> IArray<T> {
        IArray::from(self)
    }
}

impl<T: 'static> IntoPropValue<IArray<T>> for Vec<T> {
    fn into_prop_value(self) -> IArray<T> {
        IArray::from(self)
    }
}

impl<T: 'static> ImplicitClone for IArray<T> {}

impl<T: 'static> From<&'static [T]> for IArray<T> {
    fn from(a: &'static [T]) -> IArray<T> {
        IArray::Static(a)
    }
}

impl<T: 'static> From<Vec<T>> for IArray<T> {
    fn from(a: Vec<T>) -> IArray<T> {
        IArray::Rc(Rc::from(a))
    }
}

impl<T: 'static> From<Rc<[T]>> for IArray<T> {
    fn from(a: Rc<[T]>) -> IArray<T> {
        IArray::Rc(a)
    }
}

impl<T: 'static> IArray<T> {
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        match self {
            Self::Static(a) => a.iter(),
            Self::Rc(a) => a.iter(),
        }
    }

    pub fn len(&self) -> usize {
        match self {
            Self::Static(a) => a.len(),
            Self::Rc(a) => a.len(),
        }
    }
}
