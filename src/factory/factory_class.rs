pub trait Struct {
    fn say_hello(&self);
}

pub trait StructFactory {
    fn make(&self) -> impl Struct;
}

pub struct StructA {
    x: u8,
}

impl Struct for StructA {
    fn say_hello(&self) {
        println!("I am StructA x:{}", self.x)
    }
}

pub struct StructB {
    x: u8,
    y: u8,
}

impl Struct for StructB {
    fn say_hello(&self) {
        println!("I am StructB x:{} y:{}", self.x, self.y)
    }
}

pub struct StructAFactory {
    x: u8,
}

pub struct StructBFactory {
    x: u8,
    y: u8,
}

impl StructAFactory {
    pub fn new() -> StructAFactory {
        StructAFactory { x: 0 }
    }
}

impl StructFactory for StructAFactory {
    fn make(&self) -> StructA {
        StructA { x: self.x }
    }
}
