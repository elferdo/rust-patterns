trait T {
	fn apply(&self, t: impl T) -> bool;
	fn accept_a(a: A) -> bool;
	fn accept_b(b: B) -> bool;
}

struct A {}

struct B{}

impl T for A{
	fn apply(&self, t: impl T) -> bool {
		false
	}

	fn accept_a(a: A) -> bool {
		false
	}

	fn accept_b(b: B) -> bool {
		true
	}
}

impl T for B{
	fn apply(&self, t: impl T) -> bool {
		false
	}

	fn accept_a(a: A) -> bool {
		true
	}

	fn accept_b(b: B) -> bool {
		false
	}
}

fn f(a: impl T, b: impl T) -> bool {
	b.apply(a)
}

#[test]
fn xxx() {
	let a = A{};
	let b = B{};

	assert!(f(a, b));
}
