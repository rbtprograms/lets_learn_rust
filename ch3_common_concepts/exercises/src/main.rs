fn main() {
    println!("Hello, world!");
    let temp_result = f_to_c(16.0);
    println!("f_to_c is: {}", temp_result);

    let fib_result = fib(7);
    println!("fib is: {}", fib_result);
}

fn f_to_c(temp:f32) -> f32 {
    (temp - 32.0) / 1.8000
}

fn fib(n:u32) -> u32 {
    if n == 1 || n == 2 {
        return 1;
    } 
    return fib(n - 1) + fib(n - 2);
}