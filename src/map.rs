use crate::ImplicitClone;
use crate::*;
use indexmap::map::Iter as MapIter;
use indexmap::IndexMap as Map;
use std::borrow::Borrow;
use std::fmt;
use std::hash::Hash;
use std::rc::Rc;

#[derive(PartialEq)]
pub enum IMap<K: Eq + Hash + ImplicitClone + 'static, V: PartialEq + ImplicitClone + 'static> {
    Static(&'static [(K, V)]),
    Rc(Rc<Map<K, V>>),
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
    pub fn iter(&self) -> IMapIter<K, V> {
        match self {
            Self::Static(a) => IMapIter::Slice(a.iter()),
            Self::Rc(a) => IMapIter::Map(a.iter()),
            Self::Empty => IMapIter::Empty,
        }
    }

    pub fn len(&self) -> usize {
        match self {
            Self::Static(a) => a.len(),
            Self::Rc(a) => a.len(),
            Self::Empty => 0,
        }
    }

    pub fn is_empty(&self) -> bool {
        match self {
            Self::Static(a) => a.is_empty(),
            Self::Rc(a) => a.is_empty(),
            Self::Empty => true,
        }
    }

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
}

impl<V: PartialEq + ImplicitClone + 'static> IMap<IString, V> {
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
    pub fn get_static_str(&self, key: &'static str) -> Option<V> {
        match self {
            Self::Static(a) => a.iter().find_map(|(k, v)| (*k == key).then(|| v)).cloned(),
            Self::Rc(a) => a.get(key).cloned(),
            Self::Empty => None,
        }
    }
}

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

#[macro_export]
macro_rules! imap_deconstruct {
    ($(let { $($key:ident),+ $(,)? } = $map:expr;)*) => {
        $(
        $(
            let $key = $map.get_static_str(stringify!($key));
        )*
        )*
    };
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn imap_deconstruct() {
        let my_imap = [(IString::from("foo"), 1), (IString::from("bar"), 2)]
            .into_iter()
            .collect::<IMap<IString, u32>>();
        imap_deconstruct!(
            let { foo, bar, baz } = my_imap;
            let { foobarbaz } = my_imap;
        );
        assert_eq!(foo, Some(1));
        assert_eq!(bar, Some(2));
        assert_eq!(baz, None);
        assert_eq!(foobarbaz, None);
    }

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
