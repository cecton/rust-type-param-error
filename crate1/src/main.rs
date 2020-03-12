use crate2::Foo;

struct Bar;

impl Foo for Bar {
    fn foo<A, B>(x: u32, y: u32) {
        println!("Hello, world!");
    }
}

fn main() {
}
