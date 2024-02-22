trait Trait {
    fn abs(self) -> Self;
}

impl Trait for i64 {
    fn abs(self) -> Self {
        2 * self
    }
}

fn main() {
    let x = 4;
    print!("{}", x.abs());
    print!("{}", x.abs());
}
