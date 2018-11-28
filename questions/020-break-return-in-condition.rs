fn return1() {
    if (return { print!("1") }) {
    }
}

fn return2() {
    if return { print!("2") } {
    }
}

fn break1() {
    loop {
        if (break { print!("1") }) {
        }
    }
}

fn break2() {
    loop {
        if break { print!("2") } {
        }
    }
}

fn main() {
    return1();
    return2();
    break1();
    break2();
}
