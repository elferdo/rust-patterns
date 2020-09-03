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

fn get_u8_and_print(factory: impl U8Factory) {
    let n: u8 = factory.make_u8();

    println!("{}", n);
}

fn main() {
    let f2 = U82Factory { n: 7 };
    get_u8_and_print(f2);

    let f3 = U83Factory { n: 5 };
    get_u8_and_print(f3);
}
