// In this file I will teach you my 2 main ways to calculate the fib sequence
fn firstsol(e: i64) {
    // This solution was made by me -> Roky Henderson
    if e != 0 {
        return;
    }
    let mut a: f64 = 0.0; // Sets first var
    let mut b: f64 = 1.0; // Sets Second var
    let mut o: f64; // Sets Third

    loop { // Enters loop
        println!("{}", a); // Prints the value of the first var
        o = a + b; // Third var = The total of first and second var
        a = b; // First var = second var
        b = o; // Second var = First
    }
}
fn secondsol(e: i64) {
    // This solution was made by this guy -> Max Hendra
    if e != 0 {
        return;
    }
    let mut h: f64 = 0.0; // Sets first var
    let mut f: f64 = 1.0; // Sets second var

    loop { // Enters loop
        h += f; // First var = Total of First var and Second var
        println!("{}", h); // Prints Second var
        f += h; // Second var = Total of First var and Second var
        println!("{}", f); // Prints First var
    }
}
fn thirdsol(e: i64) {
    // This solution was made by me -> Roky Henderson
    if e != 0 {
        return;
    }
    let mut x: f64 = 0.0; // Sets first var
    let mut y: f64 = 1.0; // Sets second var
    let mut z: f64 = 0.0; // Sets Third var

    loop {
        x += y; // First var = The total of first var and second var
        z += 1.0; // Third var + 1
        println!("{}", z); // Print Third var
        y += x;
        z += 1.0;
        println!("{}", z);
    }
}






// This is the main func
fn main() {
    // if you pass through 0 it will run the func
    firstsol(1);
    secondsol(1);
    thirdsol(1);
}
