trait SayHi {
    ///
    fn say_hi(&self);
}

#[derive(Default)]
struct Foo;

impl Foo {
    fn bar(&self) {}
}

/*
trait Default {
    fn default() -> Self;
}
*/

// simple trait implementation
impl SayHi for Foo {
    fn say_hi(&self) {
        println!("Foo!");
    }
}

// implement a foreign trait for your own type
impl ::std::clone::Clone for Foo {
    fn clone(&self) -> Foo {
        Foo
    }
}

// traits can be implemented for types from other crates
impl SayHi for ::std::string::String {
    fn say_hi(&self) {
        println!("hi, I'm a string!");
    }
}

// traits can also be implemented for primitive types
impl SayHi for u32 {
    fn say_hi(&self) {
        println!("I'm an unsigned 32-bit integer!");
    }
}

// foreign traits *cannot* be implemented for foreign types; you have to control one or the other
/*
impl ::std::string::ToString for ::std::vec::Vec<u32> {
    fn to_string(&self) -> String {
        unimplemented!()
    }
}
*/

mod submod {
    // traits have to be in-scope to call their methods
    use crate::SayHi;

    pub(crate) fn say_hi_string() {
        String::new().say_hi();
    }
}

fn main() {
    let foo = Foo;
    foo.say_hi();
    String::new().say_hi();
    let int = 0u32;
    int.say_hi();
    submod::say_hi_string();

    let foo: Foo = Foo::default();
    let foo: Foo = Default::default();
}