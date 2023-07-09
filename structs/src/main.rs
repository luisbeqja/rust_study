struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let mut user1 = User {
        username: String::from("user1"),
        email: String::from("Luis"),
        sign_in_count: 1,
        active: true,
    };

    user1.username = String::from("user2");

    let user2 = build_user(String::from("user2"), String::from("Luis"));

    println!("Hello, {}!", user2.email);
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
