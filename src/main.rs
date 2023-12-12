fn main() {
    println!("Hello, world!");

    let mut x: i32 = 1;
    // simple loop to count to 10M
    loop {
        if x > 1e7 as i32 {
            break;
        }
        x += 1;
        println!("{}", x)
    }
}
