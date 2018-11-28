#[repr(u8)]
enum Enum {
    First,
    Second,
}

impl Enum {
    fn p(self) {
        match self {
            First => print!("1"),
            Second => print!("2"),
        }
    }
}

fn main() {
    Enum::p(unsafe {
        std::mem::transmute(1u8)
    });
}
