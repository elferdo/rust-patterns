use rust_patterns::listener::{dyn_trait, enum_container};

fn main() {
    let c = dyn_trait::container();

    for el in c {
        el.say_hello();
    }

    let d = enum_container::container();

    for el in d {
        el.say_hello();
    }
}
