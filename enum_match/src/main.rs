#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i+1)
    }
}


fn main() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    dbg!(five);
    dbg!(six);
    dbg!(none);

    println!("enum value {}", value_in_cents(Coin::Penny));
    println!("enum value {}", value_in_cents(Coin::Nickel));
    println!("enum value {}", value_in_cents(Coin::Dime));
    println!("enum value {}", value_in_cents(Coin::Quarter(UsState::Alabama)));
    println!("enum value {}", value_in_cents(Coin::Quarter(UsState::Alaska)));

}
// enum IpAddr {
//     V4(u8, u8, u8, u8),
//     V6(String),
// }

// let home = IpAddr::V4(127, 0, 0, 1);
// let loopback = IpAddr::V6(String::from("::1"));

// enum IpAddrKind {
//     V4,
//     V6,
// }

// struct IpAddr {
//     kind: IpAddrKind,
//     address: String,
// }

// let home = IpAddr {
//     kind: IpAddrKind::V4,
//     address: String::from("127.0.0.1"),
// };
// let loopback = IpAddr {
//     kind: IpAddrKind::V6,
//     address: String::from("::1"),
// };

// fn route(ip_kind:  IpAddrKind) {}

// fn main() {
    // let four = IpAddrKind::V4;
    // let six: IpAddrKind = IpAddrKind::V6;

    // route(IpAddrKind::V4);
    // route(IpAddrKind::V6);
    
    // println!("Hello, world! {}, {}",four, six);
// }
