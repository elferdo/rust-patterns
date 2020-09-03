pub struct A {}

pub struct B {}

pub trait HelloSayer {
    fn say_hello(&self);
}

impl HelloSayer for A {
    fn say_hello(&self) {
        println!("Hi, I'm A");
    }
}

impl HelloSayer for B {
    fn say_hello(&self) {
        println!("Hi, I'm B");
    }
}

pub fn container() -> Vec<Box<dyn HelloSayer>> {
    vec![Box::new(A {}), Box::new(B {})]
}
