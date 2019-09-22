struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);
fn main() {
    println!("Hello, world!");

    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    user1.email = String::from("newemail@test.com");

    println!("{}", user1.email);

    //struct update syntax
    let user2 = User {
        email: String::from("anotherstring"),
        username: String::from("yetantoherstring"),
        ..user1
    };
    println!("{}", user2.email);
    println!("{}", user2.active);

    let black = Color(0,0,0);
    let origin = Point(0,0,0);
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}