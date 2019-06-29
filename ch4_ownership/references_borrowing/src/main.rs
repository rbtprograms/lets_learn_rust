fn main() {
    let s1 = String::from("Howdy");
    //references as function parameters is called borrowing
    let len = calculate_length(&s1);
    println!("The length of {} is {}", s1, len);

    let mut s2 = String::from("hello");
    change(&mut s2);
    //     ^^^^ mutable reference
    println!("{}", s2);

    //mutable references are useful, but one big restriction
    //can only have 1 mutable reference to a piece of data in scope
    let s3 = &mut s2;
    //let s4 = &mut s2; seocnd time doesnt work
    println!("{}", s3);

    //this will work though because new scope
    {
        let s4 = &mut s2;
        println!("{}", s4)
    }
    //the bleow reference was immutable, and you CANNOT have mutable
    //and immutable references to the same value in the same scope
    //however, the below is valid because it comes after the last time 
    //the mutable reference was mentioned

    //you can either have 1 mutable reference, or as many immutable references as you would like
    let s5 = &s2;
    println!("{}", s5);

    //the below would break it because it extends the life of the mutable reference
    //println!("{}", s3);

    // let reference_to_nothing = dangle();
    let good_it_works = no_dangle();
    println!("{}", good_it_works);
}

//passing a reference allows you to use the value without transfering ownership
fn calculate_length(s: &String) -> usize {
    //this will print the actual value, not just the address like in Go
    println!("REFERENCE: {}", s);
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

//the following doesnt work because s is deallocated once the function is over
//so the refenced passed back is nothing. 
// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s
// }

//Instead return the string directly
fn no_dangle() -> String {
    let s = String::from("hello");
    s
}