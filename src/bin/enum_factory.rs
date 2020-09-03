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

enum Product {
    ProductA { n: u8 },
    ProductB { c: char, m: f32 },
}

impl Product {
    fn say_hello(&self) {
        match self {
            Product::ProductA { n } => println!("Hello, I'm Product A {}", n),
            Product::ProductB { c, m } => println!("Hello, I'm Product B {} {}", c, m),
        }
    }
}

trait ProductFactory {
    fn make_product(&self) -> Product;
}

struct AFactory {
    n: u8,
}

struct BFactory {
    c: char,
    m: f32,
}

impl ProductFactory for AFactory {
    /// We are simply copying values here, but the construction can be
    /// more costly or may fail
    fn make_product(&self) -> Product {
        Product::ProductA { n: self.n }
    }
}

impl ProductFactory for BFactory {
    /// We are simply copying values here, but the construction can be
    /// more costly or may fail
    fn make_product(&self) -> Product {
        Product::ProductB {
            c: self.c,
            m: self.m,
        }
    }
}

/// At this point we need to build something that implements Product
/// but we let others choose for us which specific product will be used.
/// The details of their construction are also hidden in the Factory
fn get_product_and_print(factory: impl ProductFactory) {
    let product = factory.make_product();

    product.say_hello();
}

fn main() {
    let f2 = AFactory { n: 5 };
    get_product_and_print(f2);

    let f3 = BFactory { c: 'f', m: 3.33 };
    get_product_and_print(f3);
}
