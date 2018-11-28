macro_rules! m {
    ($a:tt) => { print!("1") };
    ($a:tt $b:tt) => { print!("2") };
    ($a:tt $b:tt $c:tt) => { print!("3") };
    ($a:tt $b:tt $c:tt $d:tt) => { print!("4") };
    ($a:tt $b:tt $c:tt $d:tt $e:tt) => { print!("5") };
    ($a:tt $b:tt $c:tt $d:tt $e:tt $f:tt) => { print!("6") };
    ($a:tt $b:tt $c:tt $d:tt $e:tt $f:tt $g:tt) => { print!("7") };
}

fn main() {
    m!(-1);
    m!(-1.);
    m!(-1.0);
    m!(-1.0e1);
    m!(-1.0e-1);
}
