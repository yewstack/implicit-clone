use indexmap::map::Iter as MapIter;
use indexmap::map::Keys as MapKeys;
use indexmap::map::Values as MapValues;
use indexmap::IndexMap as Map;
use std::borrow::Borrow;
use std::hash::Hash;

/// An immutable hash map type inspired by [Immutable.js](https://immutable-js.com/).
///
/// This type is cheap to clone and thus implements [`ImplicitClone`]. It can be created based on a
/// `&'static [(K, V)]`, or based on a reference counted
/// [`IndexMap`](https://crates.io/crates/indexmap).
///
/// This type has the least stable API at the moment and is subject to change a lot before the 1.0
/// release.
#[cfg_attr(docsrs, doc(cfg(feature = "map")))]
#[derive(PartialEq)]
pub enum IMap<K: Eq + Hash + ImplicitClone + 'static, V: PartialEq + ImplicitClone + 'static> {
    /// A (small) static map.
    Static(&'static [(K, V)]),
    /// An reference counted map.
    Rc(Rc<Map<K, V>>),
    /// An empty map.
    Empty,
}

// TODO add insta tests
impl<
        K: fmt::Debug + Eq + Hash + ImplicitClone + 'static,
        V: fmt::Debug + PartialEq + ImplicitClone + 'static,
    > fmt::Debug for IMap<K, V>
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Static(a) => a.fmt(f),
            Self::Rc(a) => a.fmt(f),
            Self::Empty => write!(f, "{{}}"),
        }
    }
}

impl<K: Eq + Hash + ImplicitClone + 'static, V: PartialEq + ImplicitClone + 'static> Clone
    for IMap<K, V>
{
    fn clone(&self) -> Self {
        match self {
            Self::Static(a) => Self::Static(a),
            Self::Rc(a) => Self::Rc(a.clone()),
            Self::Empty => Self::Empty,
        }
    }
}

impl<K: Eq + Hash + ImplicitClone + 'static, V: PartialEq + ImplicitClone + 'static> Default
    for IMap<K, V>
{
    fn default() -> Self {
        Self::Empty
    }
}

impl<K: Eq + Hash + ImplicitClone + 'static, V: PartialEq + ImplicitClone + 'static>
    FromIterator<(K, V)> for IMap<K, V>
{
    fn from_iter<I: IntoIterator<Item = (K, V)>>(it: I) -> Self {
        let vec = it.into_iter().collect::<Map<K, V>>();
        Self::Rc(Rc::from(vec))
    }
}

impl<K: Eq + Hash + ImplicitClone + 'static, V: PartialEq + ImplicitClone + 'static> ImplicitClone
    for IMap<K, V>
{
}

impl<K: Eq + Hash + ImplicitClone + 'static, V: PartialEq + ImplicitClone + 'static>
    From<&'static [(K, V)]> for IMap<K, V>
{
    fn from(a: &'static [(K, V)]) -> IMap<K, V> {
        IMap::Static(a)
    }
}

impl<K: Eq + Hash + ImplicitClone + 'static, V: PartialEq + ImplicitClone + 'static> From<Map<K, V>>
    for IMap<K, V>
{
    fn from(a: Map<K, V>) -> IMap<K, V> {
        IMap::Rc(Rc::new(a))
    }
}

impl<K: Eq + Hash + ImplicitClone + 'static, V: PartialEq + ImplicitClone + 'static>
    From<Rc<Map<K, V>>> for IMap<K, V>
{
    fn from(a: Rc<Map<K, V>>) -> IMap<K, V> {
        IMap::Rc(a)
    }
}

impl<K: Eq + Hash + ImplicitClone + 'static, V: PartialEq + ImplicitClone + 'static> IMap<K, V> {
    /// Return an iterator over the key-value pairs of the map, in their order.
    #[inline]
    pub fn iter(&self) -> IMapIter<K, V> {
        match self {
            Self::Static(a) => IMapIter::Slice(a.iter()),
            Self::Rc(a) => IMapIter::Map(a.iter()),
            Self::Empty => IMapIter::Empty,
        }
    }

    /// Return an iterator over the keys of the map, in their order.
    #[inline]
    pub fn keys(&self) -> IMapKeys<K, V> {
        match self {
            Self::Static(a) => IMapKeys::Slice(a.iter()),
            Self::Rc(a) => IMapKeys::Map(a.keys()),
            Self::Empty => IMapKeys::Empty,
        }
    }

    /// Return an iterator over the values of the map, in their order.
    #[inline]
    pub fn values(&self) -> IMapValues<K, V> {
        match self {
            Self::Static(a) => IMapValues::Slice(a.iter()),
            Self::Rc(a) => IMapValues::Map(a.values()),
            Self::Empty => IMapValues::Empty,
        }
    }

    /// Return the number of key-value pairs in the map.
    ///
    /// Computes in **O(1)** time.
    #[inline]
    pub fn len(&self) -> usize {
        match self {
            Self::Static(a) => a.len(),
            Self::Rc(a) => a.len(),
            Self::Empty => 0,
        }
    }

    /// Returns true if the map contains no elements.
    ///
    /// Computes in **O(1)** time.
    #[inline]
    pub fn is_empty(&self) -> bool {
        match self {
            Self::Static(a) => a.is_empty(),
            Self::Rc(a) => a.is_empty(),
            Self::Empty => true,
        }
    }

    /// Return a clone to the value stored for `key`, if it is present,
    /// else `None`.
    ///
    /// Computes in **O(1)** time (average).
    #[inline]
    pub fn get<Q: ?Sized>(&self, key: &Q) -> Option<V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq,
    {
        match self {
            Self::Static(a) => a
                .iter()
                .find_map(|(k, v)| (k.borrow() == key).then(|| v))
                .cloned(),
            Self::Rc(a) => a.get(key).cloned(),
            Self::Empty => None,
        }
    }

    /// Return clones to the key-value pair stored for `key`,
    /// if it is present, else `None`.
    ///
    /// Computes in **O(1)** time (average).
    #[inline]
    pub fn get_key_value<Q: ?Sized>(&self, key: &Q) -> Option<(K, V)>
    where
        K: Borrow<Q>,
        Q: Hash + Eq,
    {
        match self {
            Self::Static(a) => a.iter().find(|(k, _)| k.borrow() == key).cloned(),
            Self::Rc(a) => a.get_key_value(key).map(|(k, v)| (k.clone(), v.clone())),
            Self::Empty => None,
        }
    }

    /// Return item index, key and value
    #[inline]
    pub fn get_full<Q: ?Sized>(&self, key: &Q) -> Option<(usize, K, V)>
    where
        K: Borrow<Q>,
        Q: Hash + Eq,
    {
        match self {
            Self::Static(a) => a
                .iter()
                .enumerate()
                .find_map(|(i, (k, v))| (k.borrow() == key).then(|| (i, k.clone(), v.clone()))),
            Self::Rc(a) => a.get_full(key).map(|(i, k, v)| (i, k.clone(), v.clone())),
            Self::Empty => None,
        }
    }

    /// Get a key-value pair by index.
    ///
    /// Valid indices are *0 <= index < self.len()*
    ///
    /// Computes in **O(1)** time.
    #[inline]
    pub fn get_index(&self, index: usize) -> Option<(K, V)> {
        match self {
            Self::Static(a) => a.get(index).cloned(),
            Self::Rc(a) => a.get_index(index).map(|(k, v)| (k.clone(), v.clone())),
            Self::Empty => None,
        }
    }

    /// Return item index, if it exists in the map.
    ///
    /// Computes in **O(1)** time (average).
    #[inline]
    pub fn get_index_of<Q: ?Sized>(&self, key: &Q) -> Option<usize>
    where
        K: Borrow<Q>,
        Q: Hash + Eq,
    {
        match self {
            Self::Static(a) => a
                .iter()
                .enumerate()
                .find_map(|(i, (k, _))| (k.borrow() == key).then(|| i)),
            Self::Rc(a) => a.get_index_of(key),
            Self::Empty => None,
        }
    }

    /// Return `true` if an equivalent to `key` exists in the map.
    ///
    /// Computes in **O(1)** time (average).
    #[inline]
    pub fn contains_key<Q: ?Sized>(&self, key: &Q) -> bool
    where
        K: Borrow<Q>,
        Q: Hash + Eq,
    {
        match self {
            Self::Static(a) => a.iter().any(|(k, _)| k.borrow() == key),
            Self::Rc(a) => a.contains_key(key),
            Self::Empty => false,
        }
    }

    /// Get the last key-value pair.
    ///
    /// Computes in **O(1)** time.
    #[inline]
    pub fn last(&self) -> Option<(K, V)> {
        match self {
            Self::Static(a) => a.last().cloned(),
            Self::Rc(a) => a.last().map(|(k, v)| (k.clone(), v.clone())),
            Self::Empty => None,
        }
    }
}

impl<V: PartialEq + ImplicitClone + 'static> IMap<IString, V> {
    #[doc(hidden)]
    #[inline]
    pub fn get_static_str(&self, key: &'static str) -> Option<V> {
        let key = IString::from(key);
        match self {
            Self::Static(a) => a.iter().find_map(|(k, v)| (*k == key).then(|| v)).cloned(),
            Self::Rc(a) => a.get(&key).cloned(),
            Self::Empty => None,
        }
    }
}

impl<V: PartialEq + ImplicitClone + 'static> IMap<&'static str, V> {
    #[doc(hidden)]
    #[inline]
    pub fn get_static_str(&self, key: &'static str) -> Option<V> {
        match self {
            Self::Static(a) => a.iter().find_map(|(k, v)| (*k == key).then(|| v)).cloned(),
            Self::Rc(a) => a.get(key).cloned(),
            Self::Empty => None,
        }
    }
}

#[allow(missing_docs)]
pub enum IMapIter<'a, K, V> {
    Slice(std::slice::Iter<'a, (K, V)>),
    Map(MapIter<'a, K, V>),
    Empty,
}

impl<'a, K: Eq + Hash + ImplicitClone + 'static, V: PartialEq + ImplicitClone + 'static> Iterator
    for IMapIter<'a, K, V>
{
    type Item = (K, V);

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::Slice(it) => it.next().map(|(k, v)| (k.clone(), v.clone())),
            Self::Map(it) => it.next().map(|(k, v)| (k.clone(), v.clone())),
            Self::Empty => None,
        }
    }
}

#[allow(missing_docs)]
pub enum IMapKeys<'a, K, V> {
    Slice(std::slice::Iter<'a, (K, V)>),
    Map(MapKeys<'a, K, V>),
    Empty,
}

impl<'a, K: Eq + Hash + ImplicitClone + 'static, V: PartialEq + ImplicitClone + 'static> Iterator
    for IMapKeys<'a, K, V>
{
    type Item = K;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::Slice(it) => it.next().map(|(k, _)| k.clone()),
            Self::Map(it) => it.next().cloned(),
            Self::Empty => None,
        }
    }
}

#[allow(missing_docs)]
pub enum IMapValues<'a, K, V> {
    Slice(std::slice::Iter<'a, (K, V)>),
    Map(MapValues<'a, K, V>),
    Empty,
}

impl<'a, K: Eq + Hash + ImplicitClone + 'static, V: PartialEq + ImplicitClone + 'static> Iterator
    for IMapValues<'a, K, V>
{
    type Item = V;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::Slice(it) => it.next().map(|(_, v)| v.clone()),
            Self::Map(it) => it.next().cloned(),
            Self::Empty => None,
        }
    }
}

#[cfg(test)]
mod test_map {
    use super::*;

    #[test]
    fn map_in_map() {
        let map_1 = [
            (IString::from("foo1"), 1),
            (IString::from("bar1"), 2),
            (IString::from("baz1"), 3),
        ]
        .into_iter()
        .collect::<IMap<IString, u32>>();
        let map_2 = [
            (IString::from("foo2"), 4),
            (IString::from("bar2"), 5),
            (IString::from("baz2"), 6),
        ]
        .into_iter()
        .collect::<IMap<IString, u32>>();
        let map_of_map = [("map_1", map_1), ("map_2", map_2)]
            .into_iter()
            .collect::<IMap<&'static str, IMap<IString, u32>>>();
        let flattened_vec = map_of_map
            .iter()
            .flat_map(|(_key, map)| map.iter().collect::<Vec<(_, _)>>())
            .collect::<Vec<(_, _)>>();
        // TODO allow PartialEq IString with &str
        assert_eq!(
            flattened_vec,
            [
                (IString::from("foo1"), 1),
                (IString::from("bar1"), 2),
                (IString::from("baz1"), 3),
                (IString::from("foo2"), 4),
                (IString::from("bar2"), 5),
                (IString::from("baz2"), 6),
            ]
        );
    }

    #[test]
    fn static_map() {
        const _MAP: IMap<&str, u32> = IMap::Static(&[("foo", 1)]);
    }
}
