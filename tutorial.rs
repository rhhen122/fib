// In this file I will teach you my 2 main ways to calculate the fib sequence
fn firstsol() {
    // This solution was made by me -> Roky Henderson
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
fn secondsol() {
    // This solution was made by this gut -> Max Hendra
    let mut h: f64; // Sets first var
    let mut f: f64 = 1.0; // Sets second var

    loop { // Enters loop
        h += f; // First var = Total of First var and Second var
        println!("{}", h); // Prints Second var
        f += h; // Second var = Total of First var and Second var
        println!("{}", f); // Prints First var
    }
}
fn thirdsol() {
    // This solution was made by me -> Roky Henderson
    let mut x: f64;
    let mut y: f64 = 1.0;
    let mut z: f64 = 0.0;

    loop {
        x += y;
        z += 1.0;
        println!("{}", z);
        y += x;
        z += 1.0;
        println!("{}", z);
    }
}






// This is the main func
fn main() {
    //firstsol()
    //secondsol()
    //thirdsol()
}