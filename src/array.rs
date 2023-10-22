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

impl<T: ImplicitClone + 'static> Extend<T> for IArray<T> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        self.insert_many(self.len(), iter);
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

    /// Makes a mutable reference into the array.
    ///
    /// If this array is an `Rc` with no other strong or weak references, returns
    /// a mutable slice of the contained data without any cloning. Otherwise, it clones the
    /// data into a new array and returns a mutable slice into that.
    ///
    /// # Example
    ///
    /// ```
    /// # use implicit_clone::unsync::*;
    /// # use std::rc::Rc;
    /// // This will reuse the Rc storage
    /// let mut v1 = IArray::<u8>::Rc(Rc::new([1,2,3]));
    /// v1.make_mut()[1] = 123;
    /// assert_eq!(&[1,123,3], v1.as_slice());
    ///
    /// // This will create a new copy
    /// let mut v2 = IArray::<u8>::Static(&[1,2,3]);
    /// v2.make_mut()[1] = 123;
    /// assert_eq!(&[1,123,3], v2.as_slice());
    /// ```
    #[inline]
    pub fn make_mut(&mut self) -> &mut [T] {
        // This code is somewhat weirdly written to work around https://github.com/rust-lang/rust/issues/54663 -
        // we can't just check if this is an Rc with one reference with get_mut in an if branch and copy otherwise,
        // since returning the mutable slice extends its lifetime for the rest of the function.
        match self {
            Self::Rc(ref mut rc) => {
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

    /// Inserts several objects into the array.
    ///
    /// This overwrites `self` to a new refcounted array with clones of the previous items,
    /// with items from the `values` iterator inserted starting at the specified index, shifting
    /// later items down.
    ///
    /// # Panics
    ///
    /// Panics if the index is greater than one more than the length of the array.
    ///
    /// # Example
    ///
    /// ```
    /// # use implicit_clone::unsync::*;
    /// let mut v = IArray::<u8>::Static(&[1,2,6]);
    /// v.insert_many(2, [3,4,5]);
    /// assert_eq!(&[1,2,3,4,5,6], v.as_slice());
    /// ```
    pub fn insert_many<I: IntoIterator<Item = T>>(&mut self, index: usize, values: I) {
        let head = self.as_slice()[..index].iter().cloned();
        let tail = self.as_slice()[index..].iter().cloned();
        let rc = head.chain(values).chain(tail).collect();
        *self = Self::Rc(rc);
    }

    /// Inserts an object into the array.
    ///
    /// This overwrites `self` to a new refcounted array with clones of the previous items,
    /// with `value` inserted at the `index`, shifting later items down.
    ///
    /// [`Self::insert_many`] will be more efficient if inserting multiple items.
    ///
    /// # Panics
    ///
    /// Panics if the index is greater than one more than the length of the array.
    ///
    /// # Example
    ///
    /// ```
    /// # use implicit_clone::unsync::*;
    /// let mut v = IArray::<u8>::Static(&[1,2,4]);
    /// v.insert(2, 3);
    /// assert_eq!(&[1,2,3,4], v.as_slice());
    /// ```
    pub fn insert(&mut self, index: usize, value: T) {
        self.insert_many(index, std::iter::once(value));
    }

    /// Adds an object to the end of the array.
    ///
    /// This overwrites `self` to a new refcounted array with clones of the previous items,
    /// with `value` added at the end.
    ///
    /// [`Self::extend`] will be more efficient if inserting multiple items.
    ///
    /// # Example
    ///
    /// ```
    /// # use implicit_clone::unsync::*;
    /// let mut v = IArray::<u8>::Static(&[1,2,3]);
    /// v.push(4);
    /// assert_eq!(&[1,2,3,4], v.as_slice());
    /// ```
    pub fn push(&mut self, value: T) {
        self.insert(self.len(), value);
    }

    /// Removes a range of items from the array.
    ///
    /// This overwrites `self` to a new refcounted array with clones of the previous items, excluding
    /// the items covered by `range`, with later items shifted up.
    ///
    /// # Panics
    ///
    /// Panics if the range is out of bounds.
    ///
    /// # Example
    ///
    /// ```
    /// # use implicit_clone::unsync::*;
    /// let mut v = IArray::<u8>::Static(&[1,2,10,20,3]);
    /// v.remove_range(2..4);
    /// assert_eq!(&[1,2,3], v.as_slice());
    /// ```
    pub fn remove_range(&mut self, range: std::ops::Range<usize>) {
        let head = self.as_slice()[..range.start].iter().cloned();
        let tail = self.as_slice()[range.end..].iter().cloned();
        let rc = head.chain(tail).collect();
        *self = Self::Rc(rc);
    }

    /// Removes an item from the array.
    ///
    /// This overwrites `self` to a new refcounted array with clones of the previous items, excluding
    /// the items at `index`, with later items shifted up.
    ///
    /// # Panics
    ///
    /// Panics if the index is out of bounds.
    ///
    /// # Example
    ///
    /// ```
    /// # use implicit_clone::unsync::*;
    /// let mut v = IArray::<u8>::Static(&[1,2,10,3]);
    /// v.remove(2);
    /// assert_eq!(&[1,2,3], v.as_slice());
    /// ```
    pub fn remove(&mut self, index: usize) {
        self.remove_range(index..index + 1)
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
    fn extend() {
        let mut array = [1, 2, 3].into_iter().collect::<IArray<u32>>();
        array.extend([4, 5, 6]);
        assert_eq!(&[1, 2, 3, 4, 5, 6], array.as_slice());
    }
}
