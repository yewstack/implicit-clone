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

    fn host_library<T: ImplicitClone>(value: &T) -> T {
        value.clone()
    }

    macro_rules! host_library {
        ($a:expr) => {
            host_library(&$a)
        };
    }

    struct NonImplicitCloneType;

    #[test]
    fn custom() {
        #[derive(Clone)]
        struct ImplicitCloneType;

        impl ImplicitClone for ImplicitCloneType {}

        host_library!(ImplicitCloneType);
    }

    #[test]
    fn copy_types() {
        fn assert_copy<T: Copy>(_: T) {}

        macro_rules! test_all {
            ($($t:ty),* $(,)?) => {
                $(host_library!(<$t>::default());)*
                $(assert_copy(<$t>::default());)*
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
        );
    }

    #[test]
    fn ref_type() {
        host_library!(&NonImplicitCloneType);
        // `host_library!(NonImplicitCloneType)` doesn't compile
    }

    #[test]
    fn option() {
        host_library!(Some("foo"));
        // `host_library!(Some(NonImplicitCloneType));` doesn't compile
    }

    #[test]
    fn tuples() {
        host_library!((1,));
        host_library!((1, 2));
        host_library!((1, 2, 3));
        host_library!((1, 2, 3, 4));
        host_library!((1, 2, 3, 4, 5));
        host_library!((1, 2, 3, 4, 5, 6));
        host_library!((1, 2, 3, 4, 5, 6, 7));
        host_library!((1, 2, 3, 4, 5, 6, 7, 8));
        host_library!((1, 2, 3, 4, 5, 6, 7, 8, 9));
        host_library!((1, 2, 3, 4, 5, 6, 7, 8, 9, 10));
        host_library!((1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11));
        host_library!((1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12));
        // `host_library!((1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13));` doesn't compile
        // `host_library!((NonImplicitCloneType,));` doesn't compile
    }
}
