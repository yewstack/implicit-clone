#![warn(missing_debug_implementations, missing_docs, unreachable_pub)]
#![allow(clippy::unnecessary_lazy_evaluations)]
#![doc(test(
    no_crate_inject,
    attr(deny(warnings, rust_2018_idioms), allow(dead_code, unused_variables))
))]
#![cfg_attr(docsrs, feature(doc_cfg))]
//! # ImplicitClone
//!
//! A library that introduces the marker trait [`ImplicitClone`](crate::ImplicitClone) which allows
//! reproducing the behavior of the trait [`Copy`][std::marker::Copy] but calls the
//! [`Clone`][std::clone::Clone] implementation instead and must be implemented in the host
//! library.
//!
//! The idea is that you must implement this trait on types that are cheap to clone
//! ([`std::rc::Rc`][std::rc::Rc] and [`std::sync::Arc`][std::sync::Arc] types are
//! automatically implemented). Then the host library must use the trait
//! [`ImplicitClone`](crate::ImplicitClone) to allow their users to pass objects that will be
//! cloned automatically.
//!
//! This crate is in the category `rust-patterns` but this is actually a Rust anti-pattern. In Rust
//! the user should always handle borrowing and ownership by themselves. Nevertheless, this pattern
//! is sometimes desirable. For example, UI frameworks that rely on propagating properties from
//! ancestors to children will always need to use Rc'ed types to allow every child component to
//! update. This is the case in React-like framework like Yew.
//!
//! This crates also provide a few convenient immutable types for handling cheap-to-clone string,
//! array and maps which you can find in the modules [`sync`](crate::sync) and
//! [`unsync`](crate::unsync). Those types implement [`ImplicitClone`](crate::ImplicitClone) and
//! hold only types that implement [`ImplicitClone`](crate::ImplicitClone) as well. **One big
//! particularity: iterating on these types yields clones of the items and not references.** This
//! can be particularly handy when using a React-like framework.
//!
//! [std::marker::Copy]: https://doc.rust-lang.org/std/marker/trait.Copy.html
//! [std::clone::Clone]: https://doc.rust-lang.org/std/clone/trait.Clone.html
//! [std::rc::Rc]: https://doc.rust-lang.org/std/rc/struct.Rc.html
//! [std::sync::Arc]: https://doc.rust-lang.org/std/sync/struct.Arc.html

/// Thread-safe version of immutable types.
pub mod sync;
/// Single-threaded version of immutable types.
pub mod unsync;

/// Marker trait for types that can be cloned implicitly.
///
/// Behaves exactly like [`Copy`] but calls the [`Clone`] implementation instead and must be
/// implemented in the host library.
pub trait ImplicitClone: Clone {}

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
    &'static str,
    (),
);

macro_rules! impl_implicit_clone_for_tuple {
    ($($param:ident),+) => {
        impl<$($param: ImplicitClone),+> ImplicitClone for ($($param),+) {}
    };
}

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

/// Same as `format!()` but generates an `IString` instead.
///
/// # Usage
///
/// ```
/// use implicit_clone::{sync::IString, iformat};
///
/// let s = iformat!("stuff={}", 42);
/// assert_eq!(s, "stuff=42");
/// ```
#[macro_export]
macro_rules! iformat {
    ($($tt:tt)*) => {
        IString::from(format!($($tt)*))
    };
}

/// Same as `vec![]` but generates an `IArray` instead.
///
/// # Usage
///
/// ```
/// use implicit_clone::{sync::IArray, iarray};
///
/// let a = iarray!["foo", "bar"];
/// assert_eq!(a, ["foo", "bar"]);
///
/// let a = iarray!["foo"; 2];
/// assert_eq!(a, ["foo", "foo"]);
/// ```
#[macro_export]
macro_rules! iarray {
    ($($tt:tt)*) => {
        IArray::Static(&[$($tt)*])
    };
}
