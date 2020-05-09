#[repr(C)]
union U {
    f1: i32,
    f2: f32,
}

fn main() {
    let u = U { f2: 1.0 };
    let x = unsafe { match u {
        U { f1: i } if i == 1 => i,
        U { f1: i } => i,
        U { f2: f } => (f + 1.0) as i32,
    }};
    print!("{}", x);
}
