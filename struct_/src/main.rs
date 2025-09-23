#[derive(Debug)]
struct User {
    username: String,
    email: String,
    password: String,
}

fn main() {
    println!("Hello, world!");

    let user1 = User {
        username: String::from("user1"),
        email: String::from("user1@example.com"),
        password: String::from("password1"),
    };

    let (username, email, password) = (user1.username, user1.email, user1.password);

    println!("username: {}", username);
    println!("email: {}", email);
    println!("password: {}", password);
}
