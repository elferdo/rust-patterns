/*
struct Callee {}
struct Caller<'c> {
    callee: Option<&'c Callee>,
}

impl Callee {
    fn say_hello(&self) {
        println!("Hi, I'm the Callee");
    }
}

impl<'c> Caller<'c> {
    fn new(c: &'c Callee) -> Caller {
        Caller { callee: Some(c) }
    }

    fn connect(&mut self, c: &'c Callee) {
        self.callee = Some(c);
    }

    fn call(&self) {
        match self.callee {
            Some(c) => c.say_hello(),
            None => println!("No callee to call"),
        }
    }
}

struct RefHolder<'r> {
    r: Option<&'r ConnectionHandle<'r>>,
}

struct Connection<'h> {
    ref_holder: &'h mut RefHolder<'h>,
}

struct ConnectionHandle<'c> {
    connection: &'c Connection<'c>,
}

impl<'h> Connection<'h> {
    fn new(h: &'h mut RefHolder<'h>) -> Self {
        let c = Connection { ref_holder: h };

        c
    }

    fn setup_handle(&'h mut self) {
        self.ref_holder.r = Some(self);
    }
}
*/

use std::rc::Rc;

struct Node<'p> {
    p: &'p Parent<'p>,
    n: u8,
}

struct Parent<'p> {
    c: Option<Node<'p>>,
}

//impl<'p> Parent<'p> {
fn make_child<'p>(parent: &'p mut Parent<'p>) {
    parent.c = Some(Node { p: &parent, n: 3 });

    //self.c = Some(n);
}
//}

fn main() {
    /*
        let callee = Callee {};
        let caller1 = Caller::new(&callee);
        let mut caller2 = Caller { callee: None };

        // let conn = Connection::new(&caller, &callee);

        caller1.call();
        caller2.call();

        caller2.connect(&callee);
        caller2.call();

        let h = RefHolder { r: None };
        let c = Connection::new(&mut h);
    */
}
