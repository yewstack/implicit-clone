#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum IString {
    Static(&'static str),
    Rc(Rc<str>),
}

impl Default for IString {
    fn default() -> Self {
        Self::Static("")
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

impl PartialEq<str> for IString {
    fn eq(&self, other: &str) -> bool {
        match self {
            Self::Static(s) => s.eq(&other),
            Self::Rc(s) => (**s).eq(other),
        }
    }
}

impl PartialEq<&str> for IString {
    fn eq(&self, other: &&str) -> bool {
        match self {
            Self::Static(s) => s.eq(other),
            Self::Rc(s) => (**s).eq(*other),
        }
    }
}

impl PartialEq<String> for IString {
    fn eq(&self, other: &String) -> bool {
        match self {
            Self::Static(s) => s.eq(&other),
            Self::Rc(s) => (**s).eq(other),
        }
    }
}

impl PartialEq<&String> for IString {
    fn eq(&self, other: &&String) -> bool {
        match self {
            Self::Static(s) => s.eq(other),
            Self::Rc(s) => (**s).eq(*other),
        }
    }
}

impl std::ops::Deref for IString {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        match self {
            Self::Static(s) => *s,
            Self::Rc(s) => &*s,
        }
    }
}

impl AsRef<str> for IString {
    fn as_ref(&self) -> &str {
        &*self
    }
}

#[cfg(test)]
mod test_string {
    use super::*;

    #[test]
    fn string_cmp() {
        assert_eq!(IString::from("foo"), "foo");
        assert_eq!(IString::from("foo"), String::from("foo"));
        assert_eq!(IString::from("foo"), &String::from("foo"));
    }

    #[test]
    fn static_string() {
        const _STRING: IString = IString::Static("foo");
    }

    #[test]
    fn deref_str() {
        assert_eq!(IString::Static("foo").to_uppercase(), "FOO");
    }
}
