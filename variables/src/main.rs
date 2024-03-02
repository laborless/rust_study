fn main() {
    // let mut x = 5;
    // println!("The value of x is: {x}");
    // x = 6;
    // println!("The value of x is: {x}");

    // let x = 5;
    // let x = x + 1;
    // {
    //     let x = x * 2;
    //     println!("The value of x in the inner scope is: {x}");
    // }
    // println!("The value of x is: {x}");

    // let mut spaces = "   ";
    // spaces = spaces.len();

    // let guess: u32 = "42".parse().expect("Not a number");

    // addition
    // let sum = 5 + 10;
    // // subtraction
    // let difference = 95.5 - 4.3;
    // //multiplication
    // let product = 4 * 30;
    // //division
    // let qutient = 56.7 / 32.3;
    // let truncated = -5 / 3;
    // //remainder
    // let remainder = 43 % 5;
    // let t = true;
    // let f: bool = false; // with explicit type annotation
    // Rust’s char type is four bytes in size and represents a Unicode Scalar Value
    // let c = 'z';
    // let z: char = 'ℤ'; // with explicit type annotation
    // let heart_eyed_cat = '😻';

    // Tuple
    // let tup: (i32, f64, u8) = (500, 6.4, 1);
    // let (x, y, z) = tup;
    // println!("The value of y is: {y}");
    // let x: (i32, f64, u8) = (500, 6.4, 1);
    // let five_hundred = x.0;
    // println!("The value of five_hundred is: {five_hundred}");
    // let six_point_four = x.1;
    // println!("The value of six_point_four is: {six_point_four}");
    // let one = x.2;
    // println!("The valud of one is: {one}");

    // Array
    // let a = [1, 2, 3, 4, 5];
    // let months = ["January", "February", "March", "April", "May", "June", "July",
    // "August", "September", "October", "November", "December"];
    // let a: [i32; 5] = [1, 2, 3, 4, 5];
    // let a = [3; 5];
    // let a = [1, 2, 3, 4, 5];
    // let first = a[0];
    // let second = a[1];
    use std::io;

    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];
    
    println!("The value of the element at index {index} is {element}")

}
