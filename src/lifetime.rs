// The returned reference will live as long as 'a, and both x and y must also live at least as long as 'a.
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

// When a struct holds references, you need lifetimes to ensure the struct doesn’t outlive the data it borrows.
struct Person<'a> {
    name: &'a str,
}

// When implementing methods for structs that hold references:
impl<'a> Person<'a> {
    fn get_name(&self) -> &'a str {
        self.name
    }
}

// Rust infers lifetimes in simple cases, especially methods with &self, One input → output gets same lifetime
impl<'a> Person<'a> {
    fn name(&self) -> &str {
        self.name // `'a` inferred
    }
}

// Static -> Global, constants, references
static GREETING: &str = "Hello, world!";
fn get_greeting() -> &'static str {
    GREETING
}

// Higher-Order Functions / Closures with Lifetimes
fn apply_fn<'a, F>(x: &'a str, f: F) -> &'a str
where
    F: Fn(&'a str) -> &'a str,
{
    f(x)
}

fn main() {
    let string1 = String::from("hello");
    let string2 = String::from("world!");
    let result = longest(&string1, &string2); // ✅ both references live long enough
    println!("The longest string is: {}", result);

    let name = String::from("Alice");
    let p = Person { name: &name }; // name must outlive p

    println!("{}", p.name());
}
