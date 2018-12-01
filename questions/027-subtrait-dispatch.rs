trait Base {
    fn method(&self) {
        print!("1");
    }
}

trait Derived: Base {
    fn method(&self) {
        print!("2");
    }
}

struct BothTraits;
impl Base for BothTraits {}
impl Derived for BothTraits {}

fn dynamic_dispatch(x: &dyn Base) {
    x.method();
}

fn static_dispatch<T: Base>(x: T) {
    x.method();
}

fn main() {
    dynamic_dispatch(&BothTraits);
    static_dispatch(BothTraits);
}
