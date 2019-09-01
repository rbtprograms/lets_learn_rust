fn main() {
    let my_string = String::from("hello world");
    let word = first_word(&my_string[..]);

    let my_string_literal = "hello world";
    let word2 = first_word(&my_string_literal[..]);

    //string literals are slices, so you can do this
    let word3 = first_word(my_string_literal);

    //alternatively, can just get slice of string
    let s = String::from("hello world");
    let hello = &s[0..5];
    let world = &s[6..11];
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    //iter converts into a collection to loop over, and enumerate converts into a tuple
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}