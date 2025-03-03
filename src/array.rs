use std::fmt;

use super::Rc;
use crate::ImplicitClone;

/// An immutable array type inspired by [Immutable.js](https://immutable-js.com/).
///
/// This type is cheap to clone and thus implements [`ImplicitClone`]. It can be created based on a
/// `&'static [T]` or based on a reference counted slice (`T`).
///
/// Since `IArray<T>` is an immutable data structure, direct modifications like adding or removing
/// elements are not possible. To make changes, you need to convert it into a `Vec<T>` using
/// `.to_vec()`, modify the vector, and then convert it back into an `IArray<T>` using
/// `IArray::from`. Here's an example demonstrating this approach:
///
/// ```rust
/// # use implicit_clone::unsync::*;
/// let iarray = IArray::from(vec![1, 2, 3]);
///
/// // Convert to Vec, modify it, then convert back to IArray
/// let mut vec = iarray.to_vec();
/// vec.push(4);
/// vec.retain(|&x| x != 2); // Remove the element `2`
/// let new_iarray = IArray::from(vec);
///
/// assert_eq!(new_iarray, IArray::from(vec![1, 3, 4]));
/// ```
///
/// This ensures that you can work with a mutable `Vec<T>` while still benefiting from
/// `IArray<T>`'s immutable properties when needed.
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
        Self::EMPTY
    }
}

impl<T: ImplicitClone + 'static> FromIterator<T> for IArray<T> {
    fn from_iter<I: IntoIterator<Item = T>>(it: I) -> Self {
        let mut it = it.into_iter();
        match it.size_hint() {
            (_, Some(0)) => Self::EMPTY,
            (_, Some(1)) => {
                if let Some(element) = it.next() {
                    Self::from([element])
                } else {
                    Self::EMPTY
                }
            }
            _ => Self::Rc(Rc::from_iter(it)),
        }
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

impl<T: ImplicitClone + 'static, const N: usize> From<[T; N]> for IArray<T> {
    fn from(a: [T; N]) -> IArray<T> {
        IArray::Rc(Rc::from(a))
    }
}

/// An iterator over the elements of an `IArray`.
#[derive(Debug)]
pub struct IArrayIter<'a, T: ImplicitClone + 'static> {
    array: &'a IArray<T>,
    left: usize,
    right: usize,
}

impl<'a, T: ImplicitClone + 'static> IArrayIter<'a, T> {
    fn new(array: &'a IArray<T>) -> Self {
        Self {
            left: 0,
            right: array.len(),
            array,
        }
    }
}

impl<'a, T: ImplicitClone + 'static> Iterator for IArrayIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.left >= self.right {
            return None;
        }
        let item = &self.array[self.left];
        self.left += 1;
        Some(item)
    }
}

impl<'a, T: ImplicitClone + 'static> DoubleEndedIterator for IArrayIter<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.left >= self.right {
            return None;
        }
        self.right -= 1;
        Some(&self.array[self.right])
    }
}

impl<T: ImplicitClone + 'static> IArray<T> {
    /// An empty array without allocation.
    pub const EMPTY: Self = Self::Static(&[]);

    /// Returns a double-ended iterator over the array.
    ///
    /// # Examples
    ///
    /// ```
    /// # use implicit_clone::unsync::*;
    /// let x = IArray::<u8>::Static(&[1, 2, 3, 4, 5, 6]);
    /// let mut iter = x.iter();
    ///
    /// assert_eq!(Some(&1), iter.next());
    /// assert_eq!(Some(&6), iter.next_back());
    /// assert_eq!(Some(&5), iter.next_back());
    /// assert_eq!(Some(&2), iter.next());
    /// assert_eq!(Some(&3), iter.next());
    /// assert_eq!(Some(&4), iter.next());
    /// assert_eq!(None, iter.next());
    /// assert_eq!(None, iter.next_back());
    /// ```
    #[inline]
    pub fn iter(&self) -> IArrayIter<T> {
        IArrayIter::new(self)
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

    /// Returns a reference of an element at a position or `None` if out of bounds.
    ///
    /// # Examples
    ///
    /// ```
    /// # use implicit_clone::unsync::*;
    /// let v = IArray::<u8>::Static(&[10, 40, 30]);
    /// assert_eq!(Some(&40), v.get(1));
    /// assert_eq!(None, v.get(3));
    /// ```
    #[inline]
    pub fn get(&self, index: usize) -> Option<&T> {
        match self {
            Self::Static(a) => a.get(index),
            Self::Rc(a) => a.get(index),
        }
    }

    /// Gets a mutable reference into the array, if there are no other references.
    ///
    /// If this array is an `Rc` with no other strong or weak references, returns
    /// a mutable slice of the contained data without any cloning. Otherwise returns
    /// `None`.
    ///
    /// # Example
    ///
    /// ```
    /// # use implicit_clone::unsync::*;
    /// # use std::rc::Rc;
    /// // This will reuse the Rc storage
    /// let mut v1 = IArray::<u8>::Rc(Rc::new([1,2,3]));
    /// v1.get_mut().unwrap()[1] = 123;
    /// assert_eq!(&[1,123,3], v1.as_slice());
    ///
    /// // Another reference will prevent exclusive access
    /// let v2 = v1.clone();
    /// assert!(v1.get_mut().is_none());
    ///
    /// // Static references are immutable
    /// let mut v3 = IArray::<u8>::Static(&[1,2,3]);
    /// assert!(v3.get_mut().is_none());
    /// ```
    #[inline]
    pub fn get_mut(&mut self) -> Option<&mut [T]> {
        match self {
            Self::Rc(ref mut rc) => Rc::get_mut(rc),
            Self::Static(_) => None,
        }
    }

    /// Makes a mutable reference into the array.
    ///
    /// If this array is an `Rc` with no other strong or weak references, returns
    /// a mutable slice of the contained data without any cloning. Otherwise, it clones the
    /// data into a new array and returns a mutable slice into that.
    ///
    /// If this array is a `Static`, it clones its elements into a new `Rc` array and returns a
    /// mutable slice into that new array.
    ///
    /// # Examples
    ///
    /// ```
    /// # use implicit_clone::unsync::*;
    /// // This will reuse the Rc storage
    /// let mut data = IArray::<u8>::from(vec![1,2,3]);
    /// data.make_mut()[1] = 123;
    /// assert_eq!(&[1,123,3], data.as_slice());
    /// assert!(matches!(data, IArray::<u8>::Rc(_)));
    /// ```
    ///
    /// ```
    /// # use implicit_clone::unsync::*;
    /// // This will create a new copy
    /// let mut data = IArray::<u8>::from(vec![1,2,3]);
    /// let other_data = data.clone();
    /// data.make_mut()[1] = 123;
    /// assert_eq!(&[1,123,3], data.as_slice());
    /// assert_eq!(&[1,2,3], other_data.as_slice());
    /// assert!(matches!(data, IArray::<u8>::Rc(_)));
    /// assert!(matches!(other_data, IArray::<u8>::Rc(_)));
    /// ```
    ///
    /// ```
    /// # use implicit_clone::unsync::*;
    /// // This will create a new copy
    /// let mut data = IArray::<u8>::Static(&[1,2,3]);
    /// let other_data = data.clone();
    /// data.make_mut()[1] = 123;
    /// assert_eq!(&[1,123,3], data.as_slice());
    /// assert_eq!(&[1,2,3], other_data.as_slice());
    /// assert!(matches!(data, IArray::<u8>::Rc(_)));
    /// assert!(matches!(other_data, IArray::<u8>::Static(_)));
    /// ```
    #[inline]
    pub fn make_mut(&mut self) -> &mut [T] {
        match self {
            Self::Rc(ref mut rc) => {
                // This code is somewhat weirdly written to work around
                // https://github.com/rust-lang/rust/issues/54663 - we can't just check if this is
                // an Rc with one reference with get_mut in an if branch and copy otherwise, since
                // returning the mutable slice extends its lifetime for the rest of the function.
                if Rc::get_mut(rc).is_none() {
                    *rc = rc.iter().cloned().collect::<Rc<[T]>>();
                }
                Rc::get_mut(rc).unwrap()
            }
            Self::Static(slice) => {
                *self = Self::Rc(slice.iter().cloned().collect());
                match self {
                    Self::Rc(rc) => Rc::get_mut(rc).unwrap(),
                    _ => unreachable!(),
                }
            }
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
    fn from_iter_is_optimized() {
        let array_0 = [].into_iter().collect::<IArray<u32>>();
        assert!(matches!(array_0, IArray::Static(_)));
        let array_2 = [1, 2].into_iter().collect::<IArray<u32>>();
        assert!(matches!(array_2, IArray::Rc(_)));
        {
            let it = [1].into_iter().filter(|x| x % 2 == 0);
            assert_eq!(it.size_hint(), (0, Some(1)));
            let array_0_to_1 = it.collect::<IArray<u32>>();
            assert!(matches!(array_0_to_1, IArray::Static(_)));
        }
        {
            let it = [2].into_iter().filter(|x| x % 2 == 0);
            assert_eq!(it.size_hint(), (0, Some(1)));
            let array_0_to_1 = it.collect::<IArray<u32>>();
            assert!(matches!(array_0_to_1, IArray::Rc(_)));
        }
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
        const _ARRAY_2: IArray<(u32, u32)> = IArray::EMPTY;
        const _ARRAY_5: IArray<(u32, u32, u32, u32, u32)> = IArray::EMPTY;
    }

    #[test]
    fn floats_in_array() {
        const _ARRAY_F32: IArray<f32> = IArray::EMPTY;
        const _ARRAY_F64: IArray<f64> = IArray::EMPTY;
    }

    #[test]
    fn from() {
        let x: IArray<u32> = IArray::EMPTY;
        let _out = IArray::from(&x);

        let _array: IArray<u32> = IArray::from(&[1, 2, 3][..]);
        let _array: IArray<u32> = IArray::from(vec![1, 2, 3]);
        let _array: IArray<u32> = IArray::from(Rc::from(vec![1, 2, 3]));
        let _array: IArray<u32> = IArray::from([1]);
    }

    #[test]
    fn recursion() {
        #[derive(Clone)]
        struct _Node {
            _children: IArray<_Node>,
        }

        impl ImplicitClone for _Node {}
    }
}
