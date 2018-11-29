
trait Base {
    fn method(&self) {print!("1")}
}

trait Derived: Base {
    fn method(&self) {print!("2")}
}

struct OnlyBase;

impl Base for OnlyBase {
    fn method(&self) {print!("3")}
}


struct BothTraits;
impl Base for BothTraits {}
impl Derived for BothTraits {}

// dynamic dispatch
fn dynamic(x: &dyn Base) {
    x.method()
}

// static dispatch
fn stat<T: Base>(x: &T) {
    x.method();
}

fn main() {
    dynamic(&OnlyBase);
    stat(&OnlyBase);
    dynamic(&BothTraits);
    stat(&BothTraits);
}