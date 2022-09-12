macro_rules! x {
    ($n: expr) => {
        let a = X($n);
    };
}

struct X(u64);

impl Drop for X {
    fn drop(&mut self) {
        print!("{}", self.0);
    }
}

fn main() {
    let a = X(1);
    x!(2);
    print!("{}", a.0);
}
