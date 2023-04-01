#[repr(u8)]
enum Enum {
    First,
    Second,
}

impl Enum {
    fn p(self) {
        match self {
            Enum::First => print!("1"),
            Enum::Second => print!("2"),
        }
    }
}

fn main() {
    Enum::p(unsafe {
        std::mem::transmute(1u8)
    });
}
