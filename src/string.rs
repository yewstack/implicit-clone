use std::borrow::Cow;
use std::fmt::Debug;
use std::str::FromStr;

/// An immutable string type inspired by [Immutable.js](https://immutable-js.com/).
///
/// This type is cheap to clone and thus implements [`ImplicitClone`]. It can be created based on a
/// `&'static str` or based on a reference counted string slice ([`str`]).
#[derive(Debug, Clone, Eq)]
pub enum IString {
    /// A static string slice.
    Static(&'static str),
    /// A reference counted string slice.
    Rc(Rc<str>),
}

impl IString {
    /// Extracts a string slice containing the entire `IString`.
    ///
    /// # Examples
    ///
    /// Basic usage:
    ///
    /// ```
    /// # use implicit_clone::unsync::IString;
    /// let s = IString::from("foo");
    ///
    /// assert_eq!("foo", s.as_str());
    /// ```
    pub fn as_str(&self) -> &str {
        match self {
            Self::Static(s) => s,
            Self::Rc(s) => s,
        }
    }

    /// Obtain the contents of [`IString`] as a [`Cow`].
    ///
    /// # Examples
    ///
    /// ```
    /// # use implicit_clone::unsync::IString;
    /// use std::borrow::Cow;
    /// let s = IString::from("foo");
    ///
    /// let cow: Cow<'_, str> = s.as_cow();
    /// ```
    pub fn as_cow(&self) -> Cow<'_, str> {
        Cow::Borrowed(self.as_str())
    }
}

impl Default for IString {
    fn default() -> Self {
        Self::Static("")
    }
}

impl ImplicitClone for IString {}

impl fmt::Display for IString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self.as_str(), f)
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

impl From<Cow<'static, str>> for IString {
    fn from(cow: Cow<'static, str>) -> Self {
        match cow {
            Cow::Borrowed(s) => IString::Static(s),
            Cow::Owned(s) => s.into(),
        }
    }
}

impl PartialEq<IString> for IString {
    fn eq(&self, other: &IString) -> bool {
        self.as_str().eq(other.as_str())
    }
}

impl PartialEq<str> for IString {
    fn eq(&self, other: &str) -> bool {
        self.as_str().eq(other)
    }
}

impl PartialEq<&str> for IString {
    fn eq(&self, other: &&str) -> bool {
        self.eq(*other)
    }
}

impl PartialEq<String> for IString {
    fn eq(&self, other: &String) -> bool {
        self.eq(other.as_str())
    }
}

impl PartialEq<&String> for IString {
    fn eq(&self, other: &&String) -> bool {
        self.eq(*other)
    }
}

impl std::ops::Deref for IString {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.as_str()
    }
}

impl AsRef<str> for IString {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::hash::Hash for IString {
    #[inline]
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::hash::Hash::hash(self.as_str(), state)
    }
}

impl std::borrow::Borrow<str> for IString {
    fn borrow(&self) -> &str {
        self.as_str()
    }
}

impl FromStr for IString {
    type Err = std::convert::Infallible;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(IString::from(String::from(value)))
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for IString {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        <str as serde::Serialize>::serialize(self, serializer)
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for IString{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        <String as serde::Deserialize>::deserialize(deserializer).map(IString::from)
    }
}

#[cfg(test)]
mod test_string {
    use super::*;

    #[test]
    fn cmp() {
        assert_eq!(IString::Static("foo"), IString::Static("foo"));
        assert_eq!(IString::Static("foo"), IString::Rc(Rc::from("foo")));
        assert_eq!(IString::Rc(Rc::from("foo")), IString::Rc(Rc::from("foo")));

        assert_ne!(IString::Static("foo"), IString::Static("bar"));
        assert_ne!(IString::Static("foo"), IString::Rc(Rc::from("bar")));
        assert_ne!(IString::Rc(Rc::from("foo")), IString::Rc(Rc::from("bar")));
    }

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

    #[test]
    fn borrow_str() {
        let map: std::collections::HashMap<_, _> = [
            (IString::Static("foo"), true),
            (IString::Rc(Rc::from("bar")), true)
        ].into_iter().collect();

        assert_eq!(map.get("foo").copied(), Some(true));
        assert_eq!(map.get("bar").copied(), Some(true));
    }

    #[test]
    fn as_cow_does_not_clone() {
        let rc_s = Rc::from("foo");
        let s = IString::Rc(Rc::clone(&rc_s));
        assert_eq!(Rc::strong_count(&rc_s), 2);
        let cow: Cow<'_, str> = s.as_cow();
        assert_eq!(Rc::strong_count(&rc_s), 2);
        // this assert exists to ensure the cow lives after the strong_count asset
        assert_eq!(cow, "foo");

    }
}
