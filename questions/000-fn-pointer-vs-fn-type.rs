
fn d<T>(_: T) {
    match std::mem::size_of::<T>() {
        n if n < 4 => print!("{}", n),
        _ => print!("4"),
    }
}

fn a<T>(f: fn(T)) {
    d(f);
}

fn main() {
    a(a::<u8>);
    a(a::<u16>);
    d(a::<u32>);
    d(a::<u64>);
}
