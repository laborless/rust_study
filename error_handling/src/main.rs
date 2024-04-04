use core::panic;
use std::{fs::File, io::ErrorKind};
fn main() {
    use std::error::Error;
use std::fs::File;

fn main() -> Result<(), Box<dyn Error>> {
    let greeting_file = File::open("hello.txt")?;

    Ok(())
}
    // let greeting_file = File::open("hello.txt")
    //     .expect("hello.txt should be included in this project");
    // let greeting_file = File::open("hello.txt").unwrap();
    // let greeting_file_result = File::open("hello.txt");

    // let greeting_file = match greeting_file_result {
    //     Ok(file) => file,
    //     // Err(error) => panic!("Problem opening the file: {:?}", error),
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => match File::create("hello.txt") {
    //             Ok(fc) => fc,
    //             Err(e) => panic!("Problem creating the file: {:?}", e),
    //         },
    //         other_error => {
    //             panic!("Problem opening the file: {:?}", other_error);
    //         }
    //     }
    // };
    // let v = vec![1, 2, 3];
    // v[99];    
    // println!("Hello, world!");
    // panic!("crash and burn");
}
