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

trait U8Factory {
    fn make_u8(&self) -> u8;
}

/// Will multiply by two the argument with which it is created
struct U82Factory {
    n: u8,
}

/// Will multiply by three the argument with which it is created
struct U83Factory {
    n: u8,
}

impl U8Factory for U82Factory {
    fn make_u8(&self) -> u8 {
        2 * self.n
    }
}

impl U8Factory for U83Factory {
    fn make_u8(&self) -> u8 {
        3 * self.n
    }
}

fn get_u8_and_print(factory: &dyn U8Factory) {
    let n: u8 = factory.make_u8();

    println!("{}", n);
}

/// Chooses the type of factory to be provided and forwards
/// the construction parameters for the delivered product
fn get_factory(n: u8, m: u8) -> Box<dyn U8Factory> {
    if n == 2 {
        Box::new(U82Factory { n: m })
    } else {
        Box::new(U83Factory { n: m })
    }
}

fn main() {
    let f2 = get_factory(2, 7);
    get_u8_and_print(&*f2);

    let f3 = get_factory(3, 5);
    get_u8_and_print(&*f3);
}
