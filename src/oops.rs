struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    let mut user2 = User {
        active: user1.active,
        username: String::from("hellow"),
        email: String::from("hello@gmail.com"),
        sign_in_count: 9,
    };
    user2.username = String::from("new_name");

    // struct update syntax
    let _user3 = User {
        email: String::from("user3@gmail.com"),
        ..user1
    };
}

fn create_user(email: String, username: String) -> User {
    return User {
        active: true,
        username,
        email,
        sign_in_count: 0,
    };
}

// unit struct
struct AlwaysEqual;

fn unit() {
    let subject = AlwaysEqual;
}

// Structs Methods
struct Rectangle {
    width: u32,
    length: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.length
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.length > other.length
    }
}

fn do_something() {
    let rect1 = Rectangle {
        width: 10,
        length: 10,
    };
    println!("The area of Rectangle is {}", rect1.area());
}

// Enums: enums give you a way of saying a value is one of a possible set of values.
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(u8, u8, u8),
}

impl Message {
    fn call(&self) {
        println!("this is calling something");
    }
}

// handling various enum cases
fn process(msg: Message) {
    // use `match` when you care about every case
    match msg {
        Message::Quit => println!("Quit message"),
        Message::Move { x, y } => println!("Move to ({}, {})", x, y),
        Message::Write(text) => println!("Text message: {}", text),
        Message::ChangeColor(r, g, b) => println!("Change color to RGB({}, {}, {})", r, g, b),
    }

    // Use `if let` when you care about one specific variant
    if let Message::Write(text) = &msg {
        println!("Text message: {}", text);
    }

    // Use `let else` when you want to extract a value and return early on failure
    let Message::Move { x, y } = msg else {
        println!("Not a Move message. Exiting.");
        return;
    };
    println!("Move to ({}, {})", x, y);
}

// The null handling in rust
fn find_user(id: u32) -> Option<String> {
    if id == 1 {
        Some("Alice".to_string())
    } else {
        None
    }
}

fn doing_something() {
    match find_user(2) {
        Some(name) => println!("Found user: {}", name),
        None => println!("User not found"),
    }

    if let Some(name) = &find_user(3) {
        println!("name of the user with id 3 is {}", name);
    } else {
        println!("No name for user with id 3");
    }
}
