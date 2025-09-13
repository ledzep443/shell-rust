#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    // Uncomment this block to pass the first stage
    print!("$ ");
    io::stdout().flush().unwrap();

    // Wait for user input
    let mut input = String::new();
    while input != "exit" {
        io::stdin().read_line(&mut input).unwrap();
        println!("{}: command not found", input.trim());
        input.clear();
        print!("$ ");
        io::stdout().flush().unwrap();
    }
}
