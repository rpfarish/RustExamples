fn main() {
    let mut i = 0;
    loop {
        println!("Hello, world!");
        println!("{}", i);
        i += 1;
        if i > 10 {
            break;
        }
    }
}
