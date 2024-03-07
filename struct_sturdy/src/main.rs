struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);
struct AlwaysEqual;

fn main() {

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    let subject = AlwaysEqual;
    // let user1 = User {
    //     active: true,
    //     username: String::from("someusername123"),
    //     email: String::from("someone@example.com"),
    //     sign_in_count: 1,
    // };

    // let mut user1 = User {
    //     active: true,
    //     username: String::from("Someusername123"),
    //     email: String::from("someone@example.com"),
    //     sign_in_count: 1,
    // };
    // user1.email = String::from("anotheremail@example.com");

    // let user2 = User {
    //     active: user1.active,
    //     username: user1.username,
    //     email: String::from("anothe@example.com"),
    //     sign_in_count: user1.sign_in_count,
    // };
    // let user2 = User {
    //     email: String::from("anothe@example.com"),
    //     ..user1
    // };
    
    //My test code...
    // println!("User1 uname: {}", user1.username);
    // println!("User2 uname: {}", user2.username);
    // user1.username = String::from("cusername");
    // println!("User1 uname: {}", user1.username);
    // println!("User2 uname: {}", user2.username);

}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}
