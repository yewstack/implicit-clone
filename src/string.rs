use std::borrow::Cow;
use std::cmp::Ordering;
use std::fmt::{self, Debug};
use std::str::FromStr;

use crate::ImplicitClone;

use super::Rc;

/// An immutable string type inspired by [Immutable.js](https://immutable-js.com/).
///
/// This type is cheap to clone and thus implements [`ImplicitClone`]. It can be created based on a
/// `&'static str` or based on a reference counted string slice ([`str`]).
#[derive(Debug, Clone)]
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

impl From<std::fmt::Arguments<'_>> for IString {
    fn from(args: std::fmt::Arguments) -> IString {
        if let Some(s) = args.as_str() {
            IString::Static(s)
        } else {
            IString::from(args.to_string())
        }
    }
}

impl From<&IString> for IString {
    fn from(s: &IString) -> IString {
        s.clone()
    }
}

macro_rules! impl_cmp_as_str {
    (PartialEq::<$type1:ty, $type2:ty>) => {
        impl_cmp_as_str!(PartialEq::<$type1, $type2>::eq -> bool);
    };
    (PartialOrd::<$type1:ty, $type2:ty>) => {
        impl_cmp_as_str!(PartialOrd::<$type1, $type2>::partial_cmp -> Option<Ordering>);
    };
    ($trait:ident :: <$type1:ty, $type2:ty> :: $fn:ident -> $ret:ty) => {
        impl $trait<$type2> for $type1 {
            fn $fn(&self, other: &$type2) -> $ret {
                $trait::$fn(AsRef::<str>::as_ref(self), AsRef::<str>::as_ref(other))
            }
        }
    };
}

impl Eq for IString {}

impl_cmp_as_str!(PartialEq::<IString, IString>);
impl_cmp_as_str!(PartialEq::<IString, str>);
impl_cmp_as_str!(PartialEq::<str, IString>);
impl_cmp_as_str!(PartialEq::<IString, &str>);
impl_cmp_as_str!(PartialEq::<&str, IString>);
impl_cmp_as_str!(PartialEq::<IString, String>);
impl_cmp_as_str!(PartialEq::<String, IString>);
impl_cmp_as_str!(PartialEq::<IString, &String>);
impl_cmp_as_str!(PartialEq::<&String, IString>);

impl Ord for IString {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        Ord::cmp(AsRef::<str>::as_ref(self), AsRef::<str>::as_ref(other))
    }
}

// Manual implementation of PartialOrd that uses Ord to ensure it is consistent, as
// recommended by clippy.
impl PartialOrd for IString {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl_cmp_as_str!(PartialOrd::<IString, str>);
impl_cmp_as_str!(PartialOrd::<str, IString>);
impl_cmp_as_str!(PartialOrd::<IString, &str>);
impl_cmp_as_str!(PartialOrd::<&str, IString>);
impl_cmp_as_str!(PartialOrd::<IString, String>);
impl_cmp_as_str!(PartialOrd::<String, IString>);
impl_cmp_as_str!(PartialOrd::<IString, &String>);
impl_cmp_as_str!(PartialOrd::<&String, IString>);

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
impl<'de> serde::Deserialize<'de> for IString {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        <String as serde::Deserialize>::deserialize(deserializer).map(IString::from)
    }
}

#[cfg(test)]
mod test_string {
    use super::*;

    //
    // Frames wrap a value with a particular syntax
    // that may not be easy to write with plain macro_rules arg types
    //

    macro_rules! frame_i_static {
        ($a:expr) => {
            IString::Static($a)
        };
    }

    macro_rules! frame_i_rc {
        ($a:expr) => {
            IString::Rc(Rc::from($a))
        };
    }

    macro_rules! frame_deref {
        ($a:expr) => {
            *$a
        };
    }

    macro_rules! frame_noop {
        ($a:expr) => {
            $a
        };
    }

    macro_rules! frame_string {
        ($a:expr) => {
            String::from($a)
        };
    }

    macro_rules! frame_string_ref {
        ($a:expr) => {
            &String::from($a)
        };
    }

    #[test]
    fn eq_ne_self() {
        macro_rules! test_one {
            ($macro:tt!, $frame1:tt!, $frame2:tt!, $a:expr, $b:expr) => {
                $macro!($frame1!($a), $frame2!($b));
            };
        }

        macro_rules! test_all_frame_combos {
            ($macro:tt!, $frame1:tt!, $frame2:tt!, $a:literal, $b:literal) => {
                // 11, 12, 21, 22 - i_static-i_static, i_static-i_rc, ...
                test_one!($macro!, $frame1!, $frame1!, $a, $b);
                test_one!($macro!, $frame2!, $frame1!, $a, $b);
                test_one!($macro!, $frame1!, $frame2!, $a, $b);
                test_one!($macro!, $frame2!, $frame2!, $a, $b);
            };
            ($macro:tt!, $a:literal, $b:literal) => {
                test_all_frame_combos!($macro!, frame_i_static!, frame_i_rc!, $a, $b);
            };
        }

        test_all_frame_combos!(assert_eq!, "foo", "foo");
        test_all_frame_combos!(assert_ne!, "foo", "bar");
    }

    #[test]
    fn cmp_self() {
        macro_rules! test_one {
            ($res:expr, $frame1:tt!, $frame2:tt!, $a:expr, $b:expr) => {
                assert_eq!($res, Ord::cmp(&$frame1!($a), &$frame2!($b)));
            };
        }

        macro_rules! test_all_frame_combos {
            ($res:expr, $frame1:tt!, $frame2:tt!, $a:literal, $b:literal) => {
                // 11, 12, 21, 22 - i_static-i_static, i_static-i_rc, ...
                test_one!($res, $frame1!, $frame1!, $a, $b);
                test_one!($res, $frame2!, $frame1!, $a, $b);
                test_one!($res, $frame1!, $frame2!, $a, $b);
                test_one!($res, $frame2!, $frame2!, $a, $b);
            };
            ($res:expr, $a:literal, $b:literal) => {
                test_all_frame_combos!($res, frame_i_static!, frame_i_rc!, $a, $b);
            };
        }

        test_all_frame_combos!(Ordering::Equal, "foo", "foo");
        test_all_frame_combos!(Ordering::Greater, "foo", "bar");
        test_all_frame_combos!(Ordering::Less, "bar", "foo");
        test_all_frame_combos!(Ordering::Greater, "foobar", "foo");
        test_all_frame_combos!(Ordering::Less, "foo", "foobar");
    }

    #[test]
    fn eq_ne_strings() {
        macro_rules! test_one {
            ($macro:tt!, $a:expr, $b:expr) => {
                $macro!($a, $b);
                $macro!($b, $a);
            };
        }

        macro_rules! test_all_frame_combos {
            ($macro:tt!, $frame1:tt!, $frame2:tt!, $a:literal, $b:literal) => {
                // 12, 21 - i_rc-deref, deref-i_rc, ..., static-string_ref, string_ref-static
                test_one!($macro!, $frame1!($a), $frame2!($b));
                test_one!($macro!, $frame2!($a), $frame1!($b));
            };
            ($macro:tt!, $frame2:tt!, $a:literal, $b:literal) => {
                test_all_frame_combos!($macro!, frame_i_rc!, $frame2!, $a, $b);
                test_all_frame_combos!($macro!, frame_i_static!, $frame2!, $a, $b);
            };
            ($macro:tt!, $a:literal, $b:literal) => {
                test_all_frame_combos!($macro!, frame_deref!, $a, $b);
                test_all_frame_combos!($macro!, frame_noop!, $a, $b);
                test_all_frame_combos!($macro!, frame_string!, $a, $b);
                test_all_frame_combos!($macro!, frame_string_ref!, $a, $b);
            };
        }

        test_all_frame_combos!(assert_eq!, "foo", "foo");
        test_all_frame_combos!(assert_ne!, "foo", "bar");
    }

    #[test]
    fn partial_cmp_strings() {
        macro_rules! test_one {
            ($res:expr, $a:expr, $b:expr) => {
                assert_eq!(Some($res), PartialOrd::partial_cmp(&$a, &$b));
            };
        }

        macro_rules! test_all_frame_combos {
            ($res:expr, $frame1:tt!, $frame2:tt!, $a:literal, $b:literal) => {
                // 12, 21 - i_rc-deref, deref-i_rc, ..., static-string_ref, string_ref-static
                test_one!($res, $frame1!($a), $frame2!($b));
                test_one!($res, $frame2!($a), $frame1!($b));
            };
            ($res:expr, $frame2:tt!, $a:literal, $b:literal) => {
                test_all_frame_combos!($res, frame_i_rc!, $frame2!, $a, $b);
                test_all_frame_combos!($res, frame_i_static!, $frame2!, $a, $b);
            };
            ($res:expr, $a:literal, $b:literal) => {
                test_all_frame_combos!($res, frame_deref!, $a, $b);
                test_all_frame_combos!($res, frame_noop!, $a, $b);
                test_all_frame_combos!($res, frame_string!, $a, $b);
                test_all_frame_combos!($res, frame_string_ref!, $a, $b);
            };
        }

        test_all_frame_combos!(Ordering::Equal, "foo", "foo");
        test_all_frame_combos!(Ordering::Greater, "foo", "bar");
        test_all_frame_combos!(Ordering::Less, "bar", "foo");
        test_all_frame_combos!(Ordering::Greater, "foobar", "foo");
        test_all_frame_combos!(Ordering::Less, "foo", "foobar");
    }

    #[test]
    fn const_string() {
        const _STRING: IString = IString::Static("foo");
    }

    #[test]
    fn deref_str() {
        assert_eq!(IString::Static("foo").to_uppercase(), "FOO");
        assert_eq!(IString::Rc(Rc::from("foo")).to_uppercase(), "FOO");
    }

    #[test]
    fn borrow_str() {
        let map: std::collections::HashMap<_, _> = [
            (IString::Static("foo"), true),
            (IString::Rc(Rc::from("bar")), true),
        ]
        .into_iter()
        .collect();

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

        // this assert exists to ensure the cow lives after the strong_count assert
        assert_eq!(cow, "foo");
    }

    #[test]
    fn from_ref() {
        let s = IString::Static("foo");
        let _out = IString::from(&s);
    }

    #[test]
    fn from_fmt_arguments() {
        let s = IString::from(format_args!("Hello World!"));
        assert!(matches!(s, IString::Static("Hello World!")));

        let name = "Jane";
        let s = IString::from(format_args!("Hello {name}!"));
        assert!(matches!(s, IString::Rc(_)));
        assert_eq!(s, "Hello Jane!");
    }
}
