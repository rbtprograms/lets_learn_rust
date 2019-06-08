fn main() {
    //value is mutable
    let mut x = 5;
    //value is immutable
    let y = 8;
    //consts are permanently immutable
    const MAX_POINTS: u32 = 100_000;
    println!("MAX value: {}", MAX_POINTS);
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    //immutable variables can be shadowed however,
    //let creates a new variable instance
    let z = 6;
    let z = z + 10;
    let z = z * 3;
    println!("Z value {}", z);
}