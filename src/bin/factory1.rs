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
    fn make_u8() -> u8;
}

struct U82Factory {}
struct U83Factory {}

impl U8Factory for U82Factory {
    fn make_u8() -> u8 {
        2u8
    }
}

impl U8Factory for U83Factory {
    fn make_u8() -> u8 {
        3u8
    }
}

fn get_u8_and_print<Factory: U8Factory>() {
    let n: u8 = Factory::make_u8();

    println!("{}", n);
}

fn main() {
    get_u8_and_print::<U82Factory>();
    get_u8_and_print::<U83Factory>();
}
