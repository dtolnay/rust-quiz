macro_rules! m {
    (1) => { print!("1") };
    ($tt:tt) => { print!("2") };
}

macro_rules! e {
    ($e:expr) => { m!($e) };
}

macro_rules! t {
    ($tt:tt) => { e!($tt); m!($tt); };
}

fn main() {
    t!(1);
}
