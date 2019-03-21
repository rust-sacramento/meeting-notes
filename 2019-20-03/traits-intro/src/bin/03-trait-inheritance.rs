use std::fmt::{Debug, Display};

trait SayHi: Debug {
    fn say_hi(&self);
}

trait AllMyCrap: AsRef<[u8]> + AsMut<[u8]> + Debug + Send + Sync {

}

impl<T> AllMyCrap for T where T: AsRef<[u8]> + AsMut<[u8]> + Debug + Send + Sync {

}

#[derive(Debug)]
struct Foo;

impl<T> SayHi for T where T: Debug {
    fn say_hi(&self) {
        println!("SayHi blanket {:?}", self);
    }
}

fn main() {
    Foo.say_hi();
}