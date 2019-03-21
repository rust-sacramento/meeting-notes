trait SayHi {
    fn say_hi(&self);
}

struct Foo;

impl SayHi for Foo {
    fn say_hi(&self) {
        println!("Foo again!");
    }
}

fn say_hi_generic<T>(greeter: &T) where T: SayHi {
    greeter.say_hi();
}

// called a blanket implementation
impl<T> SayHi for T where T: std::fmt::Display {
    fn say_hi(&self) {
        println!("SayHi blanket impl: {}", self);
    }
}

// concrete implementations cannot conflict with blanket implementation
// this is allowed with an upcoming feature called specialization
/*
impl SayHi for ::std::string::String {
    ...
}
*/


fn main() {
    say_hi_generic(&Foo);
    say_hi_generic("I'm a string!");
    say_hi_generic(0u32);
}

