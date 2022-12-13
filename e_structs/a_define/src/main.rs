#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Color(i32, i32, i32);

fn new_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

fn main() {
    let email = String::from("some@example.com");
    let username = String::from("williamzborja");
    let mut user1 = new_user(email, username);

    user1.active = false;
    println!("Hello user, {:?}!", user1);

    let user2 = User {
        active: true,
        ..user1
    };
    println!("Hello user, {:?}!", user2);
    // println!("Hello user, {}!", user1.email); invalid

    let black:Color = Color(0, 0, 0);
}
