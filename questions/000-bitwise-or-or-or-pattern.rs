fn main() {
    print!(
        "{}{}",
        matches!(Some(5|20), Some(21)) as u8,
        matches!(Some(21), Some(5|20)) as u8,
    );
}
