fn main() {
    let x = plus_one(5);
    // let x = 5;
    // let x = five();
    println!("The value of x is: {x}");
    // let y = {
    //     let x = 3;
    //     x + 1
    // };
    // println!("The value of y is: {y}");
    // let x = (let y = 6);
    // println!("Hello, world!");

    // another_function();
    // another_function_i32(5);
    // print_labeled_measurement(5,'h');
    
}
fn plus_one(mut x: i32) -> i32 { 
    x = x + 1;
    x + 1
}

// fn five() -> i32 {
//     5
// }

// fn print_labeled_measurement(value: i32, unit_label: char) {
//     println!("The measurement is: {value}{unit_label}");
// }

// fn another_function() {
//     println!("Another function.");
// }

// fn another_function_i32(x: i32) {
//     println!("The value of x is: {x}");
// }