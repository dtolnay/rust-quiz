fn main() {
    let x = |_| Some(1);
    let (|x| x) = match x(1 | 2) {
        |_| Some(2) => { print!("1"); |_| Some(1) }
        |_| Some(1) => { print!("2"); |_| Some(2) }
        |_| Some(_) => { print!("3"); |_| Some(3) }
        |_| None => { print!("4"); |_| Some(4) }
    };
    print!("{}", matches!(x(|_:[();1|2]|2|1), |_| Some(5)) as u8);
}
