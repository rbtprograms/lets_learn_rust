fn main() {
    println!("Hello, world!");
    let temp_result = f_to_c(16.0);
    println!("f_to_c is: {}", temp_result);
}

fn f_to_c(temp:f32) -> f32 {
    (temp - 32.0) / 1.8000
}
