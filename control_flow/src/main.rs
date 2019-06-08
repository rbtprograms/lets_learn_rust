fn main() {
    let number = 3;

    if number < 5 {
        println!("less than 5");
    } else {
        println!("greater than 5");
    }

    // can't do the following
    // if number {
    //     println!("Success!")
    // }
    //no truthy and falsy evalutation

    if number % 4 == 0 {
        println!("divisible by 4");
    } else if number % 3 == 0 {
        println!("divisible by 3")
    } else {
        println!("not divisible by 4 or 3");
    }

    let condition: bool = false;
    let number = if condition {
        5
    } else {
        6
    };

    println!("The value of number is: {}", number);

    //three types of loops: loop, for, while
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);

    let mut number2 = 3;

    while number2 != 0 {
        println!("{}!", number2);

        number2 -= 1;
    }

    println!("LIFTOFF");

    let a = [10,20,30,40,60];
    for element in a.iter() {
        println!("the value is: {}", element);
    }

    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}
