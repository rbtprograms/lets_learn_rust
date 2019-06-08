fn main() {
    //simple types: int, float, boolean, char
    let sum = 5 + 10;
    let remained = 43 % 6;

    //booleans with and without types
    let t = true;
    let f: bool = false;

    //has a char type as well
    //four bytes, unicode scalar value
    let c: char = 'z';
    let heart_eyed_cat = '😻';

    //compound types: tuple, arrays
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    //access values either by destructuring or directly
    let (x, y, z) = tup;
    let five_hundred = tup.0;
    println!("The value of y is: {}", y);
    println!("The value of five_hundred is: {}", five_hundred);

    //arrays are less flexible than vectors
    let arr: [i32; 5] = [1,2,3,4,5];
    //arrays are best for static lists that won't change 
    //because their sizes aren't dynamic in rust
    let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];

    //this makes an array of length 5 where all the elements are 3
    let a = [3; 5];
    let first = a[0];
    let second = a[1];
}
