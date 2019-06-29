fn main() {
    //s is valid while it is in scope and is dropped at the end of its scope
    //hardcoded string literals are immutable
    let _s = "hello";

    //the String type is allocated on the heap for dynmaic string lengths
    //this type of sting is mutable
    let mut s2 = String::from("hello");
    s2.push_str(", world");
    println!("pushed string: {}", s2);

    let x = 5;
    let y = x;
    println!("X {}", x);
    println!("Y {}", y);

    let s4 = String::from("hi");
    let s3 = s4;
    //cannot use the below line, when you copy an item in rust it
    //invalidates the previous one.
    //println!("X {}", s4);
    println!("Y {}", s3);

    //to use both do the below
    let s5 = String::from("Another string");
    let s6 = s5.clone();

    println!("s1 = {}, s2 = {}", s5, s6);

    //Ints, floats, bools, and char types all have the
    //copy attribute on them, which means they can be easily
    //copied without having to deallocate the orgiinal variable
    //and they wont try to drop like the original

    let s7 = String::from("S7 STRING"); //s7 comes into scope
    takes_ownership(s7); //s7s value moves into the function, so is no longer valid
    //println!(s7); this wont run

    let z = 10; //z comes into scope
    makes_copy(z);
    println!("{}", z); //this still works because i32 has copy attribute, so it doesn't
    //eliminate the original

    let s8 = gives_ownership(); //moves value into s8
    println!("{}", s8);

    let s9 = String::from("S9 STRING");
    //this takes ownership from s9 and moves it to s10
    let s10 = takes_and_gives_back_ownership(s9);

    //println!("{}", s9); value was moved to s10
    println!("{}", s10);

    let s11 = String::from("hello");
    let (s12, len) = calculate_length(s11);
    println!("The length of '{}' is {}", s12, len);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn gives_ownership() -> String {
    let some_string = String::from("NEWEST STRING");
    some_string
}

fn takes_and_gives_back_ownership(a_string: String) -> String {
    a_string
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}