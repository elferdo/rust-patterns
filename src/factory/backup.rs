trait Struct {
    fn say_hello(&self);
}

trait StructFactory {
    fn make<'a>(n: impl Struct + 'a) -> Box<dyn Struct + 'a>;
}

impl Struct for u8 {
    fn say_hello(&self) {
        println!("Hello! I'm {}", self);
    }
}

impl StructFactory for u8 {
    fn make<'a>(n: impl Struct + 'a) -> Box<dyn Struct + 'a> {
        Box::new(n)
    }
}

fn call_say_hello<T: StructFactory>(f: T) {
    let s = T::make(3u8);

    s.say_hello();
}
