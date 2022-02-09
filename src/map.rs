use crate::*;
use std::borrow::Borrow;
use std::collections::hash_map::Iter as MapIter;
use std::collections::HashMap as Map;
use std::hash::Hash;
use std::rc::Rc;
use yew::html::{ImplicitClone, IntoPropValue};

#[derive(Debug, PartialEq)]
pub enum IMap<K: Eq + Hash + ImplicitClone + 'static, V: PartialEq + ImplicitClone + 'static> {
    Static(&'static Map<K, V>),
    Rc(Rc<Map<K, V>>),
    Empty,
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

impl<K: Eq + Hash + ImplicitClone + 'static, V: PartialEq + ImplicitClone + 'static>
    IntoPropValue<IMap<K, V>> for &'static Map<K, V>
{
    fn into_prop_value(self) -> IMap<K, V> {
        IMap::from(self)
    }
}

impl<K: Eq + Hash + ImplicitClone + 'static, V: PartialEq + ImplicitClone + 'static>
    IntoPropValue<IMap<K, V>> for Map<K, V>
{
    fn into_prop_value(self) -> IMap<K, V> {
        IMap::from(self)
    }
}

impl<K: Eq + Hash + ImplicitClone + 'static, V: PartialEq + ImplicitClone + 'static> ImplicitClone
    for IMap<K, V>
{
}

impl<K: Eq + Hash + ImplicitClone + 'static, V: PartialEq + ImplicitClone + 'static>
    From<&'static Map<K, V>> for IMap<K, V>
{
    fn from(a: &'static Map<K, V>) -> IMap<K, V> {
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
            Self::Static(a) => IMapIter::Map(a.iter()),
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

    pub fn get<Q: ?Sized>(&self, key: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
        Q: Hash + Eq,
    {
        match self {
            Self::Static(a) => a.get(key),
            Self::Rc(a) => a.get(key),
            Self::Empty => None,
        }
    }
}

impl<V: PartialEq + ImplicitClone + 'static> IMap<IString, V> {
    pub fn get_static_str(&self, key: &'static str) -> Option<&V> {
        let key = IString::from(key);
        match self {
            Self::Static(a) => a.get(&key),
            Self::Rc(a) => a.get(&key),
            Self::Empty => None,
        }
    }
}

impl<V: PartialEq + ImplicitClone + 'static> IMap<&'static str, V> {
    pub fn get_static_str(&self, key: &'static str) -> Option<&V> {
        match self {
            Self::Static(a) => a.get(key),
            Self::Rc(a) => a.get(key),
            Self::Empty => None,
        }
    }
}

pub enum IMapIter<'a, K, V> {
    Map(MapIter<'a, K, V>),
    Empty,
}

impl<'a, K: Eq + Hash + ImplicitClone + 'static, V: PartialEq + ImplicitClone + 'static> Iterator
    for IMapIter<'a, K, V>
{
    type Item = (&'a K, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::Map(it) => it.next(),
            Self::Empty => None,
        }
    }
}

#[macro_export]
macro_rules! imap_deconstruct {
    (let { $($key:ident),+ $(,)? } = $map:expr) => {
        $(
            let $key = $map.get_static_str(stringify!($key));
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
            let { foo, bar, baz } = my_imap
        );
        assert_eq!(foo, Some(&1));
        assert_eq!(bar, Some(&2));
        assert_eq!(baz, None);
    }
}
