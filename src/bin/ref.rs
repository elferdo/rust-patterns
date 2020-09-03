fn print(n: &u8) {
    println!("{}", n);
}

fn fill_me_up(n: &mut u8) {
    *n = 5;
}

struct F<'r> {
    r: &'r u8,
}

impl<'r> F<'r> {
    fn new(n: &'r u8) -> F<'r> {
        F { r: n }
    }
}

fn main() {
    let x = 3;
    print(&x);

    let mut n = 0;
    fill_me_up(&mut n);

    let f = F::new(&n);

    print(f.r);
}
