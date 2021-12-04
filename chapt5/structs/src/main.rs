fn main() {
    println!("Hello, structs!");

    let mut email = String::from("me@emailme.com");
    let mut uname = String::from("username");
    let user = build_user(email, uname);

    email = String::from("updated@email");
    uname = String::from("updated_username");

    let user2 = new_user_from_user(email, uname, &user);

    println!("user1 email: {}", user.email);
    println!("user2 email: {}", user2.email);
}

struct User {
    username: String,
    email: String,
    active: bool,
    age: u32,
}

// Tuple structs are structs without field names, but have field types.
// They are essentially named tuples. Useful when we need the tuples to have a unique type
// The Color tuple struct and Point tuple struct look similar but are different types.
// Tuple structs behave as tuples in many cases.
struct Color(i32,i32,i32);
struct Point(i32,i32,i32);

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        age: 10,
    }
}

fn new_user_from_user(email: String, username: String, user: &User) -> User {
    User {
        email,
        username,
        ..*user
    }
}
