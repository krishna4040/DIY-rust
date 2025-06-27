// smart pointers are usually represented by structs but unlike strings and vectors they implement
// taits like deref and drop

// 1. Box -> used to store data in heap

// useful for recursive data structures
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use std::ops::Deref;

use List::{Cons, Nil};

fn main() {
    let b = Box::new(5);
    print!("{}", b);

    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    // The deref trait
    let x: i32 = 5;
    let y: &i32 = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);

    // Recursive deferecing
    let m = Box::new(String::from("Rust"));
    hello(&m);
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}

struct Mysmartpointer {}

impl Deref for Mysmartpointer {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}

// we need to use the drop fn provided in stdlib to this manual clean up before automatic cleanup
impl Drop for Mysmartpointer {
    fn drop(&mut self) {
        println!("Dropping Mysmartpointer with data `{}`!", self.0);
    }
}

// 2. Referece counting
// to have multiple owners of a single value without borrowing
// Rc<T> and Arc<T>
