use std::env;
use std::fs;

fn main() {
    let file_path = "./main.rs";
    println!("In file {file_path}");

    let contents = fs::read_to_string(file_path).expect("should have been able to find file path");

    println!("with the text: \n{contents}");

    let args: Vec<String> = env::args().collect();
    let (query, file_path) = parse_config(&args);
    print!("{}, {}", query, file_path);
}

fn parse_config(args: &[String]) -> (&str, &str) {
    let query = &args[1];
    let file_path = &args[2];

    (query, file_path)
}
