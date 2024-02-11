fn main() {
    let mut a: i32 = 1;

    loop {
        if a > 5 {
            break;
        }
        println!("{:?}", a);
        a += 1;
    }
}
