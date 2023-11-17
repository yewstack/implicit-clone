#![warn(missing_debug_implementations, missing_docs, unreachable_pub)]
#![allow(clippy::unnecessary_lazy_evaluations)]
#![allow(clippy::duplicate_mod)]
#![doc(test(
    no_crate_inject,
    attr(deny(warnings, rust_2018_idioms), allow(dead_code, unused_variables))
))]
#![cfg_attr(docsrs, feature(doc_cfg))]
//! # ImplicitClone
//!
//! This library introduces the marker trait [`ImplicitClone`](crate::ImplicitClone) intended for
//! cheap-to-clone types that should be allowed to be cloned implicitly. It enables host libraries
//! using this crate to have the syntax of [`Copy`][std::marker::Copy] while actually calling the
//! [`Clone`][std::clone::Clone] implementation instead (usually when host library does such syntax
//! in a macro).
//!
//! The idea is that you must implement this trait on your cheap-to-clone types, and then the host
//! library using the trait will allow users to pass values of your types and they will be cloned
//! automatically.
//!
//! Standard types that the [`ImplicitClone`](crate::ImplicitClone) is already implemented for:
//!
//! - [`std::rc::Rc`][std::rc::Rc]
//! - [`std::sync::Arc`][std::sync::Arc]
//! - Tuples with 1-12 elements, all of which are also [`ImplicitClone`](crate::ImplicitClone)
//! - [`Option`][std::option::Option], where inner value is [`ImplicitClone`](crate::ImplicitClone)
//! - Some built-in [`Copy`][std::marker::Copy] types, like `()`, `bool`, `&T`, etc.
//!
//! This crate is in the category `rust-patterns` but this is actually a Rust anti-pattern. In Rust
//! the user should always handle borrowing and ownership by themselves. Nevertheless, this pattern
//! is sometimes desirable. For example, UI frameworks that rely on propagating properties from
//! ancestors to multiple children will always need to use `Rc`'d types to cheaply and concisely
//! update every child component. This is the case in React-like frameworks like
//! [Yew](https://yew.rs/).
//!
//! This crate also provides a few convenient immutable types for handling cheap-to-clone strings,
//! arrays and maps, you can find them in the modules [`sync`](crate::sync) and
//! [`unsync`](crate::unsync). Those types implement [`ImplicitClone`](crate::ImplicitClone) and
//! hold only types that implement [`ImplicitClone`](crate::ImplicitClone) as well. **One big
//! particularity: iterating on these types yields clones of the items and not references.** This
//! can be particularly handy when using a React-like framework.
//!
//! ## Example
//!
//! As an example, here is an implementation of a macro called `html_input! {}` which allows its
//! user to build an `<input>` HTML node:
//!
//! ```
//! // In the host library source code:
//!
//! use implicit_clone::ImplicitClone;
//! use implicit_clone::unsync::{IArray, IString};
//!
//! macro_rules! html_input {
//!     (<input $(type={$ty:expr})? $(name={$name:expr})? $(value={$value:expr})?>) => {{
//!         let mut input = Input::new();
//!         $(input.type = $ty.into();)*
//!         $(input.name.replace($name.into());)*
//!         $(input.value.replace($value.into());)*
//!         input
//!     }}
//! }
//!
//! #[derive(Clone)]
//! pub struct Input {
//!     ty: IString,
//!     name: Option<IString>,
//!     value: Option<IString>,
//! }
//!
//! impl ImplicitClone for Input {}
//!
//! impl Input {
//!     pub fn new() -> Self {
//!         Self {
//!             ty: IString::Static("text"),
//!             name: None,
//!             value: None,
//!         }
//!     }
//! }
//!
//! impl std::fmt::Display for Input {
//!     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//!         write!(f, "<input type=\"{}\"", self.ty)?;
//!         if let Some(name) = self.name.as_ref() {
//!             write!(f, " name=\"{}\"", name)?;
//!         }
//!         if let Some(value) = self.value.as_ref() {
//!             write!(f, " value=\"{}\"", value)?;
//!         }
//!         write!(f, ">")
//!     }
//! }
//!
//! // In the user's source code:
//!
//! fn component(age: &IString) -> IArray<Input> {
//!     // `age` is implicitly cloned to the 2 different inputs
//!     let input1 = html_input!(<input name={"age"} value={age}>);
//!     let input2 = html_input!(<input name={"age"} value={age}>);
//!
//!     IArray::from(vec![input1, input2])
//! }
//!
//! let age = IString::from(20.to_string());
//! let output = component(&age);
//! let output_str = output
//!     .iter()
//!     .map(|x| x.to_string())
//!     .collect::<Vec<_>>()
//!     .join("");
//!
//! assert_eq!(
//!     output_str,
//!     r#"<input type="text" name="age" value="20"><input type="text" name="age" value="20">"#,
//! );
//! ```
//!
//! [std::marker::Copy]: https://doc.rust-lang.org/std/marker/trait.Copy.html
//! [std::clone::Clone]: https://doc.rust-lang.org/std/clone/trait.Clone.html
//! [std::rc::Rc]: https://doc.rust-lang.org/std/rc/struct.Rc.html
//! [std::sync::Arc]: https://doc.rust-lang.org/std/sync/struct.Arc.html
//! [std::option::Option]: https://doc.rust-lang.org/stable/std/option/enum.Option.html

/// Thread-safe version of immutable types.
pub mod sync;
/// Single-threaded version of immutable types.
pub mod unsync;

#[cfg(feature = "implicit-clone-derive")]
pub use implicit_clone_derive::*;

/// Marker trait for cheap-to-clone types that should be allowed to be cloned implicitly.
///
/// Enables host libraries to have the same syntax as [`Copy`] while calling the [`Clone`]
/// implementation instead.
pub trait ImplicitClone: Clone {
    /// This function is not magic; it is literally defined as
    ///
    /// ```ignore
    /// fn implicit_clone(&self) -> Self {
    ///     self.clone()
    /// }
    /// ```
    ///
    /// It is useful when you want to clone but also ensure that the type implements
    /// [`ImplicitClone`].
    ///
    /// Examples:
    ///
    /// ```
    /// use implicit_clone::ImplicitClone;
    /// let x: u32 = Default::default();
    /// let clone = ImplicitClone::implicit_clone(&x);
    /// ```
    ///
    /// ```compile_fail
    /// use implicit_clone::ImplicitClone;
    /// let x: Vec<u32> = Default::default();
    /// // does not compile because Vec<_> does not implement ImplicitClone
    /// let clone = ImplicitClone::implicit_clone(&x);
    /// ```
    #[inline]
    fn implicit_clone(&self) -> Self {
        self.clone()
    }
}

impl<T: ?Sized> ImplicitClone for &T {}

impl<T: ImplicitClone> ImplicitClone for Option<T> {}

macro_rules! impl_implicit_clone {
    ($($ty:ty),+ $(,)?) => {
        $(impl ImplicitClone for $ty {})*
    };
}

#[rustfmt::skip]
impl_implicit_clone!(
    u8, u16, u32, u64, u128,
    i8, i16, i32, i64, i128,
    f32, f64,
    bool,
    usize, isize,
    char,
    (),
);

impl<const N: usize, T: ImplicitClone> ImplicitClone for [T; N] {}

macro_rules! impl_implicit_clone_for_tuple {
    ($($param:ident),+ $(,)?) => {
        impl<$($param: ImplicitClone),+> ImplicitClone for ($($param,)+) {}
    };
}

impl_implicit_clone_for_tuple!(T1,);
impl_implicit_clone_for_tuple!(T1, T2);
impl_implicit_clone_for_tuple!(T1, T2, T3);
impl_implicit_clone_for_tuple!(T1, T2, T3, T4);
impl_implicit_clone_for_tuple!(T1, T2, T3, T4, T5);
impl_implicit_clone_for_tuple!(T1, T2, T3, T4, T5, T6);
impl_implicit_clone_for_tuple!(T1, T2, T3, T4, T5, T6, T7);
impl_implicit_clone_for_tuple!(T1, T2, T3, T4, T5, T6, T7, T8);
impl_implicit_clone_for_tuple!(T1, T2, T3, T4, T5, T6, T7, T8, T9);
impl_implicit_clone_for_tuple!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10);
impl_implicit_clone_for_tuple!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11);
impl_implicit_clone_for_tuple!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12);

/// A macro to help deconstructs maps inspired by JS.
///
/// This macro is an experiment and may change or be entirely deleted before the 1.0 release.
///
/// # Usage
///
/// ```rust
/// use implicit_clone::unsync::*;
/// use implicit_clone::imap_deconstruct;
///
/// let my_imap = [(IString::from("foo"), 1), (IString::from("bar"), 2)]
///     .into_iter()
///     .collect::<IMap<IString, u32>>();
/// imap_deconstruct!(
///     let { foo, bar, baz } = my_imap;
///     let { foobarbaz } = my_imap;
/// );
/// assert_eq!(foo, Some(1));
/// assert_eq!(bar, Some(2));
/// assert_eq!(baz, None);
/// assert_eq!(foobarbaz, None);
/// ```
#[cfg(feature = "map")]
#[cfg_attr(docsrs, doc(cfg(feature = "map")))]
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
    use super::*;

    fn assert_implicit_clone<T: ImplicitClone>() {}

    fn assert_copy<T: Copy>() {}

    struct NonImplicitCloneType;

    #[test]
    fn custom() {
        #[derive(Clone)]
        struct ImplicitCloneType;

        impl ImplicitClone for ImplicitCloneType {}

        #[allow(dead_code)]
        fn assert_ok() {
            assert_implicit_clone::<ImplicitCloneType>();
        }
    }

    #[test]
    fn copy_types() {
        macro_rules! test_all {
            ($($t:ty),* $(,)?) => {
                $(assert_implicit_clone::<$t>();)*
                $(assert_copy::<$t>();)*
            };
        }

        #[rustfmt::skip]
        test_all!(
            u8, u16, u32, u64, u128,
            i8, i16, i32, i64, i128,
            f32, f64,
            bool,
            usize, isize, char,
            (),
            [u8; 4],
            &[u8],
        );
    }

    #[test]
    fn ref_type() {
        assert_implicit_clone::<&NonImplicitCloneType>();
    }

    #[test]
    fn option() {
        assert_implicit_clone::<Option<&'static str>>();
    }

    #[test]
    fn tuples() {
        assert_implicit_clone::<(u8,)>();
        assert_implicit_clone::<(u8, u8)>();
        assert_implicit_clone::<(u8, u8, u8)>();
        assert_implicit_clone::<(u8, u8, u8, u8)>();
        assert_implicit_clone::<(u8, u8, u8, u8, u8)>();
        assert_implicit_clone::<(u8, u8, u8, u8, u8, u8)>();
        assert_implicit_clone::<(u8, u8, u8, u8, u8, u8, u8)>();
        assert_implicit_clone::<(u8, u8, u8, u8, u8, u8, u8, u8)>();
        assert_implicit_clone::<(u8, u8, u8, u8, u8, u8, u8, u8, u8)>();
        assert_implicit_clone::<(u8, u8, u8, u8, u8, u8, u8, u8, u8, u8)>();
        assert_implicit_clone::<(u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8)>();
        assert_implicit_clone::<(u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8)>();
    }
}
