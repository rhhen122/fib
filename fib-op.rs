fn main() {
    let mut a: f64 = 0.0;
    let mut b: f64 = 1.0;

    loop {
        a += b;
        println!("{}", a);
        b += a;
        println!("{}", b);
    }
}
