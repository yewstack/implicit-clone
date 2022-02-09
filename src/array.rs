use std::fmt;
use std::rc::Rc;
use yew::html::{ImplicitClone, IntoPropValue};

#[derive(PartialEq)]
pub enum IArray<T: ImplicitClone + 'static> {
    Static(&'static [T]),
    Rc(Rc<[T]>),
}

// TODO add insta tests
impl<T: fmt::Debug + ImplicitClone + 'static> fmt::Debug for IArray<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Static(a) => a.fmt(f),
            Self::Rc(a) => a.fmt(f),
        }
    }
}

impl<T: ImplicitClone + 'static> Clone for IArray<T> {
    fn clone(&self) -> Self {
        match self {
            Self::Static(a) => Self::Static(a),
            Self::Rc(a) => Self::Rc(a.clone()),
        }
    }
}

impl<T: ImplicitClone + 'static> Default for IArray<T> {
    fn default() -> Self {
        Self::Static(&[])
    }
}

impl<T: ImplicitClone + 'static> FromIterator<T> for IArray<T> {
    fn from_iter<I: IntoIterator<Item = T>>(it: I) -> Self {
        let vec = it.into_iter().collect::<Vec<T>>();
        Self::Rc(Rc::from(vec))
    }
}

impl<T: ImplicitClone + 'static> IntoPropValue<IArray<T>> for &'static [T] {
    fn into_prop_value(self) -> IArray<T> {
        IArray::from(self)
    }
}

impl<T: ImplicitClone + 'static> IntoPropValue<IArray<T>> for Vec<T> {
    fn into_prop_value(self) -> IArray<T> {
        IArray::from(self)
    }
}

impl<T: ImplicitClone + 'static> ImplicitClone for IArray<T> {}

impl<T: ImplicitClone + 'static> From<&'static [T]> for IArray<T> {
    fn from(a: &'static [T]) -> IArray<T> {
        IArray::Static(a)
    }
}

impl<T: ImplicitClone + 'static> From<Vec<T>> for IArray<T> {
    fn from(a: Vec<T>) -> IArray<T> {
        IArray::Rc(Rc::from(a))
    }
}

impl<T: ImplicitClone + 'static> From<Rc<[T]>> for IArray<T> {
    fn from(a: Rc<[T]>) -> IArray<T> {
        IArray::Rc(a)
    }
}

impl<T: ImplicitClone + 'static> IArray<T> {
    pub fn iter<'a>(&'a self) -> impl Iterator<Item = T> + 'a {
        match self {
            Self::Static(a) => a.iter().cloned(),
            Self::Rc(a) => a.iter().cloned(),
        }
    }

    pub fn len(&self) -> usize {
        match self {
            Self::Static(a) => a.len(),
            Self::Rc(a) => a.len(),
        }
    }

    pub fn as_slice(&self) -> &[T] {
        match self {
            Self::Static(a) => &a,
            Self::Rc(a) => &a,
        }
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        match self {
            Self::Static(a) => a.get(index),
            Self::Rc(a) => a.get(index),
        }
    }
}

impl<'a, T, U, const N: usize> PartialEq<&'a [U; N]> for IArray<T>
where
    T: PartialEq<U> + ImplicitClone,
{
    fn eq(&self, other: &&[U; N]) -> bool {
        match self {
            Self::Static(a) => a.eq(other),
            Self::Rc(a) => a.eq(*other),
        }
    }
}

impl<T, U, const N: usize> PartialEq<[U; N]> for IArray<T>
where
    T: PartialEq<U> + ImplicitClone,
{
    fn eq(&self, other: &[U; N]) -> bool {
        match self {
            Self::Static(a) => a.eq(other),
            Self::Rc(a) => a.eq(other),
        }
    }
}

impl<T, U> PartialEq<[U]> for IArray<T>
where
    T: PartialEq<U> + ImplicitClone,
{
    fn eq(&self, other: &[U]) -> bool {
        match self {
            Self::Static(a) => a.eq(&other),
            Self::Rc(a) => a.eq(other),
        }
    }
}

impl<'a, T, U> PartialEq<&'a [U]> for IArray<T>
where
    T: PartialEq<U> + ImplicitClone,
{
    fn eq(&self, other: &&[U]) -> bool {
        match self {
            Self::Static(a) => a.eq(other),
            Self::Rc(a) => a.eq(*other),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn array_in_array() {
        let array_1 = [1, 2, 3].into_iter().collect::<IArray<u32>>();
        let array_2 = [4, 5, 6].into_iter().collect::<IArray<u32>>();
        let array_of_array = [array_1, array_2]
            .into_iter()
            .collect::<IArray<IArray<u32>>>();
        assert_eq!(array_of_array, [[1, 2, 3], [4, 5, 6]]);
    }
}
