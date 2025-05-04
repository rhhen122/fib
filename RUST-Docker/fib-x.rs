fn main() {
    let mut a: f64 = 0.0;
    let mut b: f64 = 1.0;
    let mut i: f64 = 0.0;

    loop {
        a += b;
        i += 1.0;
        println!("{}", i);
        b += a;
        i += 1.0;
        println!("{}", i);
    }
}
