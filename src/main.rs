fn main() {
    // rust uses snake case for vairables and function

    // First program
    println!("Hello world!");

    // concept of mutability
    let y = 7; // declared imutable, cannot change value
    let mut z = 5; // declared mutable, can change value

    // Const variables, uppercase
    const HOUR: i32 = 26; // are scoped

    // Shadowing
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }
    println!("The value of x is: {x}");

    // Datatypes
    // Scalar types --> Integer(signed and unsigned) & floating point & boolean & chars
    let decimal: i32 = 87_22;
    let hex: i32 = 0xff;
    let oct: i32 = 0o66;
    let bin: i32 = 0b1100;
    let byte: u8 = b'A';
    // compound types --> tuples and arrays
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (a, b, c) = tup;
    let a1 = tup.0;
    let a2 = tup.1;
    let a3 = tup.2;

    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let first = a[0];
    let second = a[1];
}

fn another_function(x: i32) -> i32 {
    // function = statements + Expressions
    println!("The value of x is: {x}");
    return 5;
}

fn control_flow() {
    let number = 6;
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {number}");

    // loop = while ture
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2; // You can also return from inside a loop. While break only exits the current loop, return always exits the current function.
        }
    };
    println!("The result is {result}");

    // loop with condition = while
    let mut number = 3;
    while number != 0 {
        println!("{number}!");
        number -= 1;
    }
    println!("LIFTOFF!!!");

    // for and array
    let a = [10, 20, 30, 40, 50];
    for element in a {
        println!("the value is: {element}");
    }
}

fn scope() {
    // chaper 4
    //A scope is the range within a program for which an item is valid. Take the following variable:
    let s = "string";

    {
        // s is not valid here, it’s not yet declared
        let s = "hello"; // s is valid from this point forward

        // do stuff with s
    } // this scope is now over, and s is no longer valid

    // when a variable goes out of scope rust calls a drop function for it

    let s1 = String::from("hello");
    let _s2 = s1;
    // s1 becmes invvalid

    // The original string thus immediately goes out of scope. Rust will run the drop function on it and its memory will be freed right away. When we print the value at the end, it will be "ahoy, world!".
    let mut sr = String::from("hello");
    sr = String::from("ahoy");

    println!("{sr}, world!");
}

fn take_ownership(s: String) {
    println!("Inside function: {s}");
} // s dropped here

fn borrow_ownership(s: &String) {
    println!("Borrowed: {s}");
} // borrow ends here, no drop

fn ownership_functions() {
    let s = String::from("hello");

    take_ownership(s);
    // s is moved, cannot use s here anymore

    let s2 = String::from("world");
    borrow_ownership(&s2);
    // s2 still valid after function call
}

// The &s1 syntax lets us create a reference that refers to the value of s1 but does not own it. Because the reference does not own it, the value it points to will not be dropped when the reference stops being used.
fn borrowing() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{s1}' is {len}.");
}

fn calculate_length(s: &String) -> usize {
    s.len()
    // if we want to modify the borrowed value we need to use mutable references
    // one big restrinction: if you have one mutable reference to a value, you cannot have other references to that value
}

fn dangle() -> &String {
    // dangle returns a reference to a String

    let s = String::from("hello"); // s is a new String

    &s // we return a reference to the String, s
} // Here, s goes out of scope, and is dropped, so its memory goes away.
// Danger! sol: return the string

mod generics_traits;
mod io;
mod module_1;
mod module_2;
mod oops;
use std::collections::HashMap;

// Special collections
fn special() {
    let mut v = Vec::new();
    v.push(10);
    v.push(20);

    let mut s = String::from("Hello");
    s.push_str(" World");

    let mut scores = HashMap::new();
    scores.insert("Alice", 10);
    scores.insert("Bob", 15);
}

// error handeling
use std::fs::File;
use std::io::{self, Read};
use std::num::ParseIntError;

fn read_and_parse_number(path: &str) -> Result<i32, Box<dyn std::error::Error>> {
    // 1. Use `File::open`, handle with `match` to access the error
    let file_result = File::open(path);

    let mut file = match file_result {
        Ok(f) => f,
        Err(e) => {
            println!("Error opening file: {}", e); // full access to error
            return Err(Box::new(e));
        }
    };

    // 2. Use `?` operator to propagate error upward
    let mut contents = String::new();
    file.read_to_string(&mut contents)?; // if this fails, function returns early with error

    // 3. Use `if let` when only handling success case
    if let Ok(_) = contents.parse::<i32>() {
        println!("Contents can be parsed to a number");
    } else {
        println!("Contents are not a valid number"); // no access to error here
    }

    // 4. Use `let else` for early return
    let Ok(num) = contents.trim().parse::<i32>() else {
        println!("Returning early due to invalid number");
        return Err("Invalid number".into());
    };

    // 5. Optional: panic! if number is too large (unrecoverable)
    if num > 1000 {
        panic!("This number is too big!"); // Unrecoverable, crashes the program
    }

    Ok(num) // return parsed number if all went well
}

fn new_main() {
    match read_and_parse_number("input.txt") {
        Ok(n) => println!("Parsed number: {}", n),
        Err(e) => println!("Operation failed: {}", e),
    }
}

mod lifetime;

// functional programming - closures and iterators
// Closures are anonymous function (i.e., no name) that can capture variables from the environment where it’s defined.

fn closure() {
    let num = 5;
    let add_num_v1 = |x: u32| -> u32 { x + 1 };
    let add_num_v2 = |x| x + num; // closure captures `num`
    println!("{}", add_num_v2(10)); // prints 15

    // when types are not defined the compiler annotates how they are used and annotates a type but if
    // we try to use another type we face compiler issue
    let example_closure = |x| x;
    let s = example_closure(String::from("hello"));
    let n = example_closure(5);

    // there are 3 traits associated with closures implemented in rust stdlib
    // 1. Fn - by reference
    // 2. FnMut - by mut reference
    // 3. FnOnce - by value

    // closures capturing env
    // 1. by value
    let name = String::from("Krishna");
    let greet = move || println!("Hello, {}!", name); // takes ownership

    greet(); // name is now moved into the closure

    // 2. by reference
    let message = String::from("Hello");

    // This closure borrows `message`
    let print_message = || println!("{}", message);

    print_message(); // ✅ can call multiple times
    print_message(); // ✅ again — because it's just borrowing

    // 3. by mutable reference
    let mut count = 0;

    // Mutably borrow `count`
    let mut increment = || {
        count += 1;
        println!("count = {}", count);
    };

    increment(); // count = 1
    increment(); // count = 2

    // ⚠️ `count` is mutably borrowed, so cannot be used directly here
}

// Implemtation in rust stdlib
// trait Iterator {
//     type Item;
//     fn next(&mut self) -> Option<Self::Item>;
// }

#[test]
fn iterator_demonstration() {
    let v1 = vec![1, 2, 3];

    let mut v1_iter = v1.iter();

    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);
}

fn process_numbers(numbers: Vec<i32>) {
    // Step 1: Map - double each number
    let mapped: Vec<i32> = numbers.iter().map(|x| x * 2).collect();
    println!("Mapped (doubled): {:?}", mapped);

    // Step 2: Filter - keep only even numbers (this is redundant here since doubling always gives even)
    let filtered: Vec<i32> = mapped.iter().cloned().filter(|x| x % 2 == 0).collect();
    println!("Filtered (evens): {:?}", filtered);

    // Step 3: Find - first number > 10
    let found = filtered.iter().find(|&&x| x > 10);
    match found {
        Some(val) => println!("Found (>10): {}", val),
        None => println!("Found: None"),
    }

    // Step 4: Final collect (already collected into `filtered`)
    println!("Final Collected: {:?}", filtered);
}
// 🔁 Loops vs. Iterators: Performance

// Two versions of a search function:
//     for loop version: manually iterates through characters/lines/words.
//     iterator version: uses .iter().filter() or .find() etc.

// Benchmark Result:
// for loop:    ~19,620,300 ns/iter
// iterator:    ~19,234,900 ns/iter

// Conclusion:
// 👉 Iterator version is just as fast, even a bit faster, but within the margin of error.
// Rust’s iterators are "zero-cost abstractions".
// This means: they look high-level, but compile down to optimized machine code, like hand-written loops.

mod concurrency;
mod smart_pointers;
