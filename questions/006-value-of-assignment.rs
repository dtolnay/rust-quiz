use std::mem;

fn main() {
    let a;
    let a = a = true;
    print!("{}", mem::size_of_val(&a));
}
