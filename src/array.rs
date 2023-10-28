use std::fmt;

use super::Rc;
use crate::ImplicitClone;

/// An immutable array type inspired by [Immutable.js](https://immutable-js.com/).
///
/// This type is cheap to clone and thus implements [`ImplicitClone`]. It can be created based on a
/// `&'static [T]` or based on a reference counted slice (`T`).
#[derive(PartialEq, Eq)]
pub enum IArray<T: ImplicitClone + 'static> {
    /// A static slice.
    Static(&'static [T]),
    /// A reference counted slice.
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

impl<T: ImplicitClone + 'static> From<&IArray<T>> for IArray<T> {
    fn from(a: &IArray<T>) -> IArray<T> {
        a.clone()
    }
}

impl<T: ImplicitClone + 'static> IArray<T> {
    /// Returns an iterator over the slice.
    ///
    /// # Examples
    ///
    /// ```
    /// # use implicit_clone::unsync::*;
    /// let x = IArray::<u8>::Static(&[1, 2, 4]);
    /// let mut iterator = x.iter();
    ///
    /// assert_eq!(iterator.next(), Some(1));
    /// assert_eq!(iterator.next(), Some(2));
    /// assert_eq!(iterator.next(), Some(4));
    /// assert_eq!(iterator.next(), None);
    /// ```
    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = T> + '_ {
        match self {
            Self::Static(a) => a.iter().cloned(),
            Self::Rc(a) => a.iter().cloned(),
        }
    }

    /// Returns the number of elements in the vector, also referred to
    /// as its 'length'.
    ///
    /// # Examples
    ///
    /// ```
    /// # use implicit_clone::unsync::*;
    /// let a = IArray::<u8>::Static(&[1, 2, 3]);
    /// assert_eq!(a.len(), 3);
    /// ```
    #[inline]
    pub fn len(&self) -> usize {
        match self {
            Self::Static(a) => a.len(),
            Self::Rc(a) => a.len(),
        }
    }

    /// Returns `true` if the vector contains no elements.
    ///
    /// # Examples
    ///
    /// ```
    /// # use implicit_clone::unsync::*;
    /// let v = IArray::<u8>::default();
    /// assert!(v.is_empty());
    ///
    /// let v = IArray::<u8>::Static(&[1]);
    /// assert!(!v.is_empty());
    /// ```
    #[inline]
    pub fn is_empty(&self) -> bool {
        match self {
            Self::Static(a) => a.is_empty(),
            Self::Rc(a) => a.is_empty(),
        }
    }

    /// Extracts a slice containing the entire array.
    ///
    /// Equivalent to `&s[..]`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use implicit_clone::unsync::*;
    /// use std::io::{self, Write};
    /// let buffer = IArray::<u8>::Static(&[1, 2, 3, 5, 8]);
    /// io::sink().write(buffer.as_slice()).unwrap();
    /// ```
    #[inline]
    pub fn as_slice(&self) -> &[T] {
        match self {
            Self::Static(a) => a,
            Self::Rc(a) => a,
        }
    }

    /// Returns a clone of an element at a position or `None` if out of bounds.
    ///
    /// # Examples
    ///
    /// ```
    /// # use implicit_clone::unsync::*;
    /// let v = IArray::<u8>::Static(&[10, 40, 30]);
    /// assert_eq!(Some(40), v.get(1));
    /// assert_eq!(None, v.get(3));
    /// ```
    #[inline]
    pub fn get(&self, index: usize) -> Option<T> {
        match self {
            Self::Static(a) => a.get(index).cloned(),
            Self::Rc(a) => a.get(index).cloned(),
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

impl<T> std::ops::Deref for IArray<T>
where
    T: ImplicitClone,
{
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        self.as_slice()
    }
}

#[cfg(feature = "serde")]
impl<T: serde::Serialize + ImplicitClone> serde::Serialize for IArray<T> {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        <[T] as serde::Serialize>::serialize(self, serializer)
    }
}

#[cfg(feature = "serde")]
impl<'de, T: serde::Deserialize<'de> + ImplicitClone> serde::Deserialize<'de> for IArray<T> {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        <Vec<T> as serde::Deserialize>::deserialize(deserializer).map(IArray::<T>::from)
    }
}

#[cfg(test)]
mod test_array {
    use super::*;

    #[test]
    fn array_in_array() {
        let array_1 = [1, 2, 3].into_iter().collect::<IArray<u32>>();
        let array_2 = [4, 5, 6].into_iter().collect::<IArray<u32>>();
        let array_of_array = [array_1, array_2]
            .into_iter()
            .collect::<IArray<IArray<u32>>>();
        assert_eq!(array_of_array, [[1, 2, 3], [4, 5, 6]]);
    }

    #[test]
    fn array_holding_rc_items() {
        struct Item;
        let _array = [Rc::new(Item)].into_iter().collect::<IArray<Rc<Item>>>();
    }

    #[test]
    fn static_array() {
        const _ARRAY: IArray<u32> = IArray::Static(&[1, 2, 3]);
    }

    #[test]
    fn deref_slice() {
        assert!(IArray::Static(&[1, 2, 3]).contains(&1));
    }

    #[test]
    fn tuple_in_array() {
        const _ARRAY_2: IArray<(u32, u32)> = IArray::Static(&[]);
        const _ARRAY_5: IArray<(u32, u32, u32, u32, u32)> = IArray::Static(&[]);
    }

    #[test]
    fn floats_in_array() {
        const _ARRAY_F32: IArray<f32> = IArray::Static(&[]);
        const _ARRAY_F64: IArray<f64> = IArray::Static(&[]);
    }

    #[test]
    fn from() {
        let x: IArray<u32> = IArray::Static(&[]);
        let _out = IArray::from(&x);
    }
}
