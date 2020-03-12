use std::fmt::Debug;

pub trait Foo {
    fn foo<A, B>(x: u32, y: impl Debug);
}
