#[derive(Debug)]
enum UsState {
    Oregon,
    Washington,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
    Half_Dollar,
}

fn main() {
    value_in_cents(Coin::Quarter(UsState::Oregon));

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("{:?}", six);

    let some_u8_value = 5;
    //use _ to cathc other cases in a match
    match some_u8_value {
        1 => println!("one"),
        5 => println!("five"),
        9 => println!("nine"),
        _ => (),
    }

    //if let syntax is a rust idiomatic coding style
    //works the say way match does, matches a pattern to an expression
    if let 5 = some_u8_value {
        println!("Its a match!");
    }
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky Penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}", state);   
            25
        },
        Coin::Half_Dollar => 50,
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}