mod str {
    pub fn len(_: &str) -> usize {
        0
    }
}

fn main() {
    let s = "hello";
    println!("{} {}", str::is_empty(s), str::len(s));
}
