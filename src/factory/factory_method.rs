pub struct MyStruct {}

impl MyStruct {
    pub fn new() -> Self {
        MyStruct {}
    }

    pub fn say_hello(&self) {
        println!("I am MyStruct");
    }
}

pub fn make_my_struct() -> MyStruct {
    MyStruct {}
}
