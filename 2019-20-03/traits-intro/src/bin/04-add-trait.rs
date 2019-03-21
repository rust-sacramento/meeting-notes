use std::ops::Add;

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq, Ord, Eq)]
struct Int(i32);

impl Add<Int> for Int {
    type Output = Int;

    fn add(self, rhs: Int) -> Self::Output {
        Int(self.0 + rhs.0)
    }
}

impl<'a, 'b> Add<&'a Int> for &'b Int {
    type Output = Int;

    fn add(self, rhs: &Int) -> Self::Output {
        Int(self.0 + rhs.0)
    }
}

impl<'a> Add<Int> for &'a Int {
    type Output = Int;

    fn add(self, rhs: Int) -> Self::Output {
        Int(self.0 + rhs.0)
    }
}

impl<'a> Add<&'a Int> for Int {
    type Output = Int;

    fn add(self, rhs: &Int) -> Self::Output {
        Int(self.0 + rhs.0)
    }
}

fn main() {
    println!("{:?}", Int(0) + Int(1));
    println!("{:?}", (&Int(2)) + Int(3));
    println!("{:?}", Int(4) + (&Int(5)));
    println!("{:?}", (&Int(6)) + (&Int(7)));
}