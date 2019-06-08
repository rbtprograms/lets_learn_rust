fn main() {
    println!("Hello, world!");
    another_function(7);

    let x = 5;

    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y);

    let z = five();
    println!("The value of z is: {}", z);

    let z = plus_one(14);
    println!("The value of z is: {}", z);
}

//function parameters MUST have type declarations
fn another_function(x: i32) {
    println!("The value of x is: {}", x);
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}