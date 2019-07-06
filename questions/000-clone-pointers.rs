use std::rc::Rc;

struct A;

fn d<T>(t: T) {
    match std::mem::size_of_val(&t) {
        0 => print!("0"),
        _ => print!("1"),
    }
}

fn main() {
    let a = &A;
    d(a);
    d(a.clone());
    
    let b = &();
    d(b);
    d(b.clone());
    
    let c = Rc::new(());
    d(Rc::clone(&c));
    d(c.clone());
}
