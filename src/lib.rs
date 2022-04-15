/// Thread-safe version of immutable types.
pub mod sync;
/// Single-threaded version of immutable types.
pub mod unsync;

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
    &'static str,
);

#[cfg(feature = "map")]
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
    #[test]
    #[cfg(feature = "map")]
    fn imap_deconstruct() {
        use crate::unsync::*;

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
}
