pub trait Bar {
	fn bar() -> i32;
}

pub struct Foo;

impl Bar for Foo {
	fn bar() -> i32 {
		54
	}
}
