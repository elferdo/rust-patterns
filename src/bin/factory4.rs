//! In some situations we may want to make choices about the creation
//! of an object at one place, and then defer the creation itself to
//! another place. Factory patterns help in this situations.
//!
//! There are two basic choices to make:
//! * What to create (type)
//! * How to create it (procedure and parameters)
//!
//! Depending on which of this information is available at which point
//! different variations of the Factory pattern will be advisable.

trait Product {
    fn say_hello(&self);
}

struct ProductA {}

impl Product for ProductA {
    fn say_hello(&self) {
        println!("Hello, I'm Product A");
    }
}

struct ProductB {}

impl Product for ProductB {
    fn say_hello(&self) {
        println!("Hello, I'm Product B");
    }
}

trait ProductFactory {
    fn make_product(&self) -> Box<dyn Product>;
}

struct AFactory {}

struct BFactory {}

impl ProductFactory for AFactory {
    fn make_product(&self) -> Box<dyn Product> {
        Box::new(ProductA {})
    }
}

impl ProductFactory for BFactory {
    fn make_product(&self) -> Box<dyn Product> {
        Box::new(ProductB {})
    }
}

fn get_u8_and_print(factory: &dyn ProductFactory) {
    let product = factory.make_product();

    product.say_hello();
}

fn main() {
    let f2 = AFactory {};
    get_u8_and_print(&f2);

    let f3 = BFactory {};
    get_u8_and_print(&f3);
}
