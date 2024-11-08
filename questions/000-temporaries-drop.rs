struct NoisyDrop;
impl Drop for NoisyDrop {
    fn drop(&mut self) {
        print!("0");
    }
}

fn main() {
    {
        let _ = &NoisyDrop;
        print!("1");
    }
    {
        _ = &NoisyDrop;
        print!("1");
    }
}
