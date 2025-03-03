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
    /// A single element.
    Single([T; 1]),
}

// TODO add insta tests
impl<T: fmt::Debug + ImplicitClone + 'static> fmt::Debug for IArray<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Static(a) => a.fmt(f),
            Self::Rc(a) => a.fmt(f),
            Self::Single(x) => x.fmt(f),
        }
    }
}

impl<T: ImplicitClone + 'static> Clone for IArray<T> {
    fn clone(&self) -> Self {
        match self {
            Self::Static(a) => Self::Static(a),
            Self::Rc(a) => Self::Rc(a.clone()),
            Self::Single(x) => Self::Single(x.clone()),
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
                    Self::Single([element])
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

impl<T: ImplicitClone + 'static> From<[T; 1]> for IArray<T> {
    fn from(a: [T; 1]) -> IArray<T> {
        IArray::Single(a)
    }
}

/// An iterator over the elements of an `IArray`.
#[derive(Debug)]
pub struct Iter<T: ImplicitClone + 'static> {
    array: IArray<T>,
    left: usize,
    right: usize,
}

impl<T: ImplicitClone + 'static> Iter<T> {
    fn new(array: IArray<T>) -> Self {
        Self {
            left: 0,
            right: array.len(),
            array,
        }
    }
}

impl<T: ImplicitClone + 'static> Iterator for Iter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.left >= self.right {
            return None;
        }
        let item = self.array.get(self.left);
        self.left += 1;
        item
    }
}

impl<T: ImplicitClone + 'static> DoubleEndedIterator for Iter<T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.left >= self.right {
            return None;
        }
        self.right -= 1;
        self.array.get(self.right)
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
    /// assert_eq!(Some(1), iter.next());
    /// assert_eq!(Some(6), iter.next_back());
    /// assert_eq!(Some(5), iter.next_back());
    /// assert_eq!(Some(2), iter.next());
    /// assert_eq!(Some(3), iter.next());
    /// assert_eq!(Some(4), iter.next());
    /// assert_eq!(None, iter.next());
    /// assert_eq!(None, iter.next_back());
    /// ```
    #[inline]
    pub fn iter(&self) -> Iter<T> {
        Iter::new(self.clone())
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
            Self::Single(_) => 1,
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
            Self::Single(_) => false,
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
            Self::Single(a) => a,
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
            Self::Single(a) if index == 0 => Some(a[0].clone()),
            Self::Single(_) => None,
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
    ///
    /// // Single items always return a mutable reference
    /// let mut v4 = IArray::<u8>::Single([1]);
    /// assert!(v4.get_mut().is_some());
    /// ```
    #[inline]
    pub fn get_mut(&mut self) -> Option<&mut [T]> {
        match self {
            Self::Rc(ref mut rc) => Rc::get_mut(rc),
            Self::Static(_) => None,
            Self::Single(ref mut a) => Some(a),
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
    /// If this array is a `Single` element, the inner array is returned directly.
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
    ///
    /// ```
    /// # use implicit_clone::unsync::*;
    /// // This will use the inner array directly
    /// let mut data = IArray::<u8>::Single([1]);
    /// let other_data = data.clone();
    /// data.make_mut()[0] = 123;
    /// assert_eq!(&[123], data.as_slice());
    /// assert_eq!(&[1], other_data.as_slice());
    /// assert!(matches!(data, IArray::<u8>::Single(_)));
    /// assert!(matches!(other_data, IArray::<u8>::Single(_)));
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
            Self::Single(array) => array,
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
            Self::Single(a) if N == 1 => a[0].eq(&other[0]),
            Self::Single(_) => false,
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
            Self::Single(a) if N == 1 => a[0].eq(&other[0]),
            Self::Single(_) => false,
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
            Self::Single(a) => a.eq(other),
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
            Self::Single(a) => a.eq(*other),
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
        let array_1 = [1].into_iter().collect::<IArray<u32>>();
        assert!(matches!(array_1, IArray::Single(_)));
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
            assert!(matches!(array_0_to_1, IArray::Single(_)));
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
}
