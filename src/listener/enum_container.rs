pub enum Listener {
    A,
    B,
}

impl Listener {
    pub fn say_hello(&self) {
        match self {
            Listener::A => println!("Hi, I'm A"),
            Listener::B => println!("Hi, I'm B"),
        }
    }
}

pub fn container() -> Vec<Listener> {
    vec![Listener::A, Listener::B]
}
