![Rust](https://github.com/yewstack/implicit-clone/actions/workflows/rust.yml/badge.svg)
[![Latest Version](https://img.shields.io/crates/v/implicit-clone.svg)](https://crates.io/crates/implicit-clone)
![License](https://img.shields.io/crates/l/implicit-clone)
[![Docs.rs](https://docs.rs/implicit-clone/badge.svg)](https://docs.rs/implicit-clone)
[![LOC](https://tokei.rs/b1/github/yewstack/implicit-clone)](https://github.com/yewstack/implicit-clone)
[![Dependency Status](https://deps.rs/repo/github/yewstack/implicit-clone/status.svg)](https://deps.rs/repo/github/yewstack/implicit-clone)

<!-- cargo-rdme start -->

# ImplicitClone

This library introduces the marker trait [`ImplicitClone`](https://docs.rs/implicit-clone/latest/implicit_clone/trait.ImplicitClone.html) intended for
cheap-to-clone types that should be allowed to be cloned implicitly. It enables host libraries
using this crate to have the syntax of [`Copy`][std::marker::Copy] while actually calling the
[`Clone`][std::clone::Clone] implementation instead (usually when host library does such syntax
in a macro).

The idea is that you must implement this trait on your cheap-to-clone types, and then the host
library using the trait will allow users to pass values of your types and they will be cloned
automatically.

Standard types that the [`ImplicitClone`](https://docs.rs/implicit-clone/latest/implicit_clone/trait.ImplicitClone.html) is already implemented for:

- [`std::rc::Rc`][std::rc::Rc]
- [`std::sync::Arc`][std::sync::Arc]
- Tuples with 1-12 elements, all of which are also [`ImplicitClone`](https://docs.rs/implicit-clone/latest/implicit_clone/trait.ImplicitClone.html)
- [`Option`][std::option::Option], where inner value is [`ImplicitClone`](https://docs.rs/implicit-clone/latest/implicit_clone/trait.ImplicitClone.html)
- Some built-in [`Copy`][std::marker::Copy] types, like `()`, `bool`, `&T`, etc.

This crate is in the category `rust-patterns` but this is actually a Rust anti-pattern. In Rust
the user should always handle borrowing and ownership by themselves. Nevertheless, this pattern
is sometimes desirable. For example, UI frameworks that rely on propagating properties from
ancestors to multiple children will always need to use `Rc`'d types to cheaply and concisely
update every child component. This is the case in React-like frameworks like
[Yew](https://yew.rs/).

This crate also provides a few convenient immutable types for handling cheap-to-clone strings,
arrays and maps, you can find them in the modules [`sync`](https://docs.rs/implicit-clone/latest/implicit_clone/sync/) and
[`unsync`](https://docs.rs/implicit-clone/latest/implicit_clone/unsync/). Those types implement [`ImplicitClone`](https://docs.rs/implicit-clone/latest/implicit_clone/trait.ImplicitClone.html) and
hold only types that implement [`ImplicitClone`](https://docs.rs/implicit-clone/latest/implicit_clone/trait.ImplicitClone.html) as well. **One big
particularity: iterating on these types yields clones of the items and not references.** This
can be particularly handy when using a React-like framework.

[std::marker::Copy]: https://doc.rust-lang.org/std/marker/trait.Copy.html
[std::clone::Clone]: https://doc.rust-lang.org/std/clone/trait.Clone.html
[std::rc::Rc]: https://doc.rust-lang.org/std/rc/struct.Rc.html
[std::sync::Arc]: https://doc.rust-lang.org/std/sync/struct.Arc.html
[std::option::Option]: https://doc.rust-lang.org/stable/std/option/enum.Option.html

<!-- cargo-rdme end -->
