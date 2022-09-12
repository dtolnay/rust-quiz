fn call(mut f: impl FnMut() + Copy) {
    f();
}

fn g(mut f: impl FnMut() + Copy) {
    f();
    call(f);
    f();
    call(f);
}

fn main() {
    let mut i = 0i32;
    g(move || {
        i += 1;
        print!("{}", i);
    });
}
