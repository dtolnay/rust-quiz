struct Drop0;
impl Drop for Drop0 {
    fn drop(&mut self) {
        print!("0");
    }
}

fn main() {
    {
        let _ = &Drop0;
        print!("1");
    }
    {
        _ = &Drop0;
        print!("1");
    }
}
