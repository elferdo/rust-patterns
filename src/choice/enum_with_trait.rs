use crate::common::hellosayer::HelloSayer;

pub enum HelloSayerEnum {
    A,
    B,
}

impl HelloSayer for HelloSayerEnum {
    fn say_hello(&self) {
        match &self {
            Self::A => println!("Hello! I'm A"),
            Self::B => println!("Hello! I'm B"),
        }
    }
}

pub fn choose(c: char) -> HelloSayerEnum {
    match c {
        'a' => HelloSayerEnum::A {},
        _ => HelloSayerEnum::B {},
    }
}
