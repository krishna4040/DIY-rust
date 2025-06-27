use std::fmt::{Debug, Display};

// 1️⃣ Define a trait
trait Summary {
    fn summarize(&self) -> String;
}

// 2️⃣ Two types implementing the trait
struct NewsArticle {
    headline: String,
    author: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{} by {}", self.headline, self.author)
    }
}

struct Tweet {
    username: String,
    content: String,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("@{}: {}", self.username, self.content)
    }
}

// 3️⃣ Trait as parameter using `impl Trait`
fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// 4️⃣ Trait bound using `+`
fn print_summary_and_display(item: &(impl Summary + Display)) {
    println!("Summary: {}", item.summarize());
    println!("Display: {}", item);
}

// 5️⃣ Generic with multiple bounds using `where`
fn debug_and_display<T, U>(t: &T, u: &U)
where
    T: Display + Clone,
    U: Debug + Clone,
{
    println!("t = {}", t);
    println!("u = {:?}", u);
}

// 6️⃣ Return type using `impl Trait`
fn get_summary_item() -> impl Summary {
    Tweet {
        username: String::from("rustacean"),
        content: String::from("Rust is awesome!"),
    }
}

// 7️⃣ Conditional method implementation
struct Pair<T> {
    x: T,
    y: T,
}

// constructor implementation example
impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Pair { x, y }
    }
}

// Only available if T implements Display + PartialOrd
impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("x is larger: {}", self.x);
        } else {
            println!("y is larger: {}", self.y);
        }
    }
}

// 8️⃣ Blanket implementation
impl<T: Display> Summary for T {
    fn summarize(&self) -> String {
        format!("(Display summary) {}", self)
    }
}

// 9️⃣ Main function to tie everything together
fn main() {
    let article = NewsArticle {
        headline: String::from("Rust Hits 1.75!"),
        author: String::from("Ferris"),
    };

    let tweet = get_summary_item(); // returns impl Summary

    notify(&article); // uses trait as parameter
    notify(&tweet); // impl Trait in return
    debug_and_display(&"Debug me", &vec![1, 2, 3]); // using where clause

    let pair = Pair::new(10, 20);
    pair.cmp_display(); // conditionally implemented method

    // Blanket implementation: i32 implements Display, so also Summary now
    let num = 42;
    println!("Blanket: {}", num.summarize());
}
