trait Or {
    fn f(self);
}

struct T;

impl Or for &T {
    fn f(self) {
        print!("1");
    }
}

impl Or for &&&&T {
    fn f(self) {
        print!("2");
    }
}

fn main() {
    let t = T;
    let wt = &T;
    let wwt = &&T;
    let wwwt = &&&T;
    let wwwwt = &&&&T;
    let wwwwwt = &&&&&T;
    t.f();
    wt.f();
    wwt.f();
    wwwt.f();
    wwwwt.f();
    wwwwwt.f();
}
