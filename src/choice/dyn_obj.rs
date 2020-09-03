use crate::common::hellosayer::HelloSayer;

struct A {}

impl HelloSayer for A {
    fn say_hello(&self) {
        println!("Hello! I'm A");
    }
}

fn get_a() -> impl HelloSayer {
    A {}
}

struct B {}

impl HelloSayer for B {
    fn say_hello(&self) {
        println!("Hello! I'm B");
    }
}

fn get_b() -> impl HelloSayer {
    B {}
}

pub fn choose(c: char) -> Box<dyn HelloSayer> {
    if c == 'a' {
        Box::new(get_a())
    } else {
        Box::new(get_b())
    }
}
