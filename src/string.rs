use std::fmt;
use std::rc::Rc;
use yew::html::{ImplicitClone, IntoPropValue};
use yew::virtual_dom::AttrValue;

#[derive(Clone, PartialEq)]
pub enum IString {
    Static(&'static str),
    Rc(Rc<str>),
}

impl Default for IString {
    fn default() -> Self {
        Self::Static("")
    }
}

impl IntoPropValue<IString> for &'static str {
    fn into_prop_value(self) -> IString {
        IString::from(self)
    }
}

impl IntoPropValue<IString> for String {
    fn into_prop_value(self) -> IString {
        IString::from(self)
    }
}

impl IntoPropValue<IString> for AttrValue {
    fn into_prop_value(self) -> IString {
        match self {
            Self::Static(s) => IString::from(s),
            Self::Owned(s) => IString::from(s),
            Self::Rc(s) => IString::from(s),
        }
    }
}

impl IntoPropValue<Option<AttrValue>> for &IString {
    fn into_prop_value(self) -> Option<AttrValue> {
        Some(match self {
            IString::Static(s) => AttrValue::Static(s),
            IString::Rc(s) => AttrValue::Rc(s.clone()),
        })
    }
}

impl ImplicitClone for IString {}

impl fmt::Display for IString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Static(s) => s.fmt(f),
            Self::Rc(s) => s.fmt(f),
        }
    }
}

impl From<&'static str> for IString {
    fn from(s: &'static str) -> IString {
        IString::Static(s)
    }
}

impl From<String> for IString {
    fn from(s: String) -> IString {
        IString::Rc(Rc::from(s))
    }
}

impl From<Rc<str>> for IString {
    fn from(s: Rc<str>) -> IString {
        IString::Rc(s)
    }
}
