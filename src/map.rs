use std::collections::hash_map::Iter as MapIter;
use std::collections::HashMap as Map;
use std::hash::Hash;
use std::rc::Rc;
use yew::html::{ImplicitClone, IntoPropValue};

#[derive(PartialEq)]
pub enum IMap<K: Eq + Hash + 'static, V: PartialEq + 'static> {
    Static(&'static Map<K, V>),
    Rc(Rc<Map<K, V>>),
    Empty,
}

impl<K: Eq + Hash + 'static, V: PartialEq + 'static> Clone for IMap<K, V> {
    fn clone(&self) -> Self {
        match self {
            Self::Static(a) => Self::Static(a),
            Self::Rc(a) => Self::Rc(a.clone()),
            Self::Empty => Self::Empty,
        }
    }
}

impl<K: Eq + Hash + 'static, V: PartialEq + 'static> Default for IMap<K, V> {
    fn default() -> Self {
        Self::Empty
    }
}

impl<K: Eq + Hash + 'static, V: PartialEq + 'static> FromIterator<(K, V)> for IMap<K, V> {
    fn from_iter<I: IntoIterator<Item = (K, V)>>(it: I) -> Self {
        let vec = it.into_iter().collect::<Map<K, V>>();
        Self::Rc(Rc::from(vec))
    }
}

impl<K: Eq + Hash + 'static, V: PartialEq + 'static> IntoPropValue<IMap<K, V>>
    for &'static Map<K, V>
{
    fn into_prop_value(self) -> IMap<K, V> {
        IMap::from(self)
    }
}

impl<K: Eq + Hash + 'static, V: PartialEq + 'static> IntoPropValue<IMap<K, V>> for Map<K, V> {
    fn into_prop_value(self) -> IMap<K, V> {
        IMap::from(self)
    }
}

impl<K: Eq + Hash + 'static, V: PartialEq + 'static> ImplicitClone for IMap<K, V> {}

impl<K: Eq + Hash + 'static, V: PartialEq + 'static> From<&'static Map<K, V>> for IMap<K, V> {
    fn from(a: &'static Map<K, V>) -> IMap<K, V> {
        IMap::Static(a)
    }
}

impl<K: Eq + Hash + 'static, V: PartialEq + 'static> From<Map<K, V>> for IMap<K, V> {
    fn from(a: Map<K, V>) -> IMap<K, V> {
        IMap::Rc(Rc::new(a))
    }
}

impl<K: Eq + Hash + 'static, V: PartialEq + 'static> From<Rc<Map<K, V>>> for IMap<K, V> {
    fn from(a: Rc<Map<K, V>>) -> IMap<K, V> {
        IMap::Rc(a)
    }
}

impl<K: Eq + Hash + 'static, V: PartialEq + 'static> IMap<K, V> {
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
}

pub enum IMapIter<'a, K, V> {
    Map(MapIter<'a, K, V>),
    Empty,
}

impl<'a, K: Eq + Hash + 'static, V: PartialEq + 'static> Iterator for IMapIter<'a, K, V> {
    type Item = (&'a K, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::Map(it) => it.next(),
            Self::Empty => None,
        }
    }
}
