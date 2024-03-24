fn main() {
    // Hash Map study (dictionary)
    use std::collections::HashMap;

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);

    // use std::collections::HashMap;

    // let field_name = String::from("Favorite color");
    // let field_value = String::from("Blue");

    // let mut map = HashMap::new();
    // map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point, try using them and
    // see what compiler error you get!

    // use std::collections::HashMap;

    // let mut scores = HashMap::new();

    // scores.insert(String::from("Blue"), 10);
    // scores.insert(String::from("Yellow"), 50);

    // for (key, value) in &scores {
    //     println!("{key}: {value}");
    // }

    // use std::collections::HashMap;

    // let mut scores = HashMap::new();

    // scores.insert(String::from("Blue"), 10);
    // scores.insert(String::from("Yellow"), 50);

    // let team_name = String::from("Blue");
    // let score = scores.get(&team_name).copied().unwrap_or(0);

    println!("Bye, HASH")
    // string study .. Stiring is implemeted as a wrapper aroud a vector of bytes
    // let hello = "Здравствуйте";

    // let s = &hello[0..4];
    // // println!("{s}");
    // for c in "Зд".chars() {
    //     println!("{c}");
    // }    
    // for b in "Зд".bytes() {
    //     println!("{b}");
    // }
    
    // let hello = String::from("Hola");
    // let hello = "Здравствуйте";
    // let answer = &hello[0];

    // let s1 = String::from("hello");
    // let h = s1[0];

    // let s1 = String::from("tic");
    // let s2 = String::from("tac");
    // let s3 = String::from("toe");
    // let s = format!("{s1}-{s2}-{s3}");
    // let s1 = String::from("tic");
    // let s2 = String::from("tac");
    // let s3 = String::from("toe");
    // let s = s1 + "-" + &s2 + "-" + &s3;
    // println!("s: {}", s);
    // let s1 = String::from("Hello, ");
    // let s2 = String::from("world!");
    // let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
    // println!("{}", s3);
    // let mut s = String::from("lo");
    // s.push('l');

    // let mut s1 = String::from("foo");
    // let s2 = "bar";
    // s1.push_str(s2);
    // println!("s2 is {s2}");

    // let mut s = String::from("foo");
    // s.push_str("bar");
    // println!("s is {}", s);

    // let data = "initial contents";

    // let s = data.to_string();

    // // the method also works on a literal directly:
    // let s = "initial contents".to_string();

    // let hello = String::from("السلام عليكم");
    // let hello = String::from("Dobrý den");
    // let hello = String::from("Hello");
    // let hello = String::from("שָׁלוֹם");
    // let hello = String::from("नमस्ते");
    // let hello = String::from("こんにちは");
    // let hello = String::from("안녕하세요");
    // let hello = String::from("你好");
    // let hello = String::from("Olá");
    // let hello = String::from("Здравствуйте");
    // let hello = String::from("Hola");

    // let mut s = String::new();

    // let mut s = String::new();

    // println!("Bye, string")

    // Vector study
    // enum SpreadsheetCell {
    //     Int(i32),
    //     Float(f64),
    //     Text(String),
    // }

    // let row = vec![
    //     SpreadsheetCell::Int(3),
    //     SpreadsheetCell::Text(String::from("blue")),
    //     SpreadsheetCell::Float(10.12),
    // ];

    // let mut v = vec![100, 32, 57];
    // for i in &mut v {
    //     *i += 50;
    // }
    // let v = vec![100, 32, 57];
    // for i in &v {
    //     println!("{i}");
    // }
    // let mut v = vec![1, 2, 3, 4, 5];
    // let first = &v[0];
    // v.push(6);
    // println!("The first element is: {first}");
    // let v = vec![1,2,3,4,5];

    // let does_not_exist = &v[100];
    // let does_not_exist = v.get(100);
    // let third: &i32 = &v[2];
    // println!("The third element is: {third}");

    // let third: Option<&i32> = v.get(2);
    // match third {
    //     Some(third) => println!("The third element is: {third}"),
    //     None => println!("There is no third element"),
    // }    

    // let v: Vec<i32> = Vec::new();
    // let mut v = vec![1, 2, 3];
    // v.push(5);
    // v.push(6);
    // v.push(7);
    // v.push(8);
    // println!("Bye, Vector!");
}
