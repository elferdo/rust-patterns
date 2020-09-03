use std::fmt::{Debug, Formatter};

#[derive(Clone, Copy)]
enum E {
    A,
    B,
}

impl Debug for E {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        match self {
            E::A => write!(f, "{}", "E::A"),
            E::B => write!(f, "{}", "E::B"),
        }
    }
}

fn main() {
    let mut m = [E::A; 3];

    let p: *mut E = &mut m[0];

    unsafe {
        *p = E::B;
    }

    dbg!(m);
}
