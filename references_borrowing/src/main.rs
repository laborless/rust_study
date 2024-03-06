fn main() {

    let reference_to_nothing = dangle();
    // let mut s = String::from("hello"); // mutable variable

    // let r1 = &s;
    // let r2 = &s;
    // println!("{}, {}", r1, r2);
    // let r3 = &mut s;
    // println!("{}", r3);
    // println!("{}, {}, {}", r1, r2, r3);

    // {
    //     let r1 = &mut s;
    // }
    // let r2 = &mut s;
    // println!("{}, {}", r1, r2);

    // change(&mut s); //mutable reference
    
    // let s1 = String::from("hello");
    // let len = calculate_length(&s1);

    // println!("The length of '{}' is {}.", s1, len);
}

fn dangle() -> &String {
    let s = String::from("hello");

    &s
}

// fn change(some_string: &mut String) {
//     some_string.push_str(", world");
// }

// fn calculate_length(s: &String) -> usize {
//     s.len()
// }
