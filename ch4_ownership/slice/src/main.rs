fn main() {
    let mut s = String::from("hello world");
    let word = first_word(&s);
    s.clear();
    println!("Value of word {}", word);
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    //iter converts into a collection to loop over, and enumerate converts into a tuple
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}