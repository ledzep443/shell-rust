#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    // Uncomment this block to pass the first stage
    print!("$ ");
    io::stdout().flush().unwrap();

    // Wait for user input
    let mut input = String::new();
    loop {
        io::stdin().read_line(&mut input).unwrap();
        if input.trim() == "exit 0" { 
            break;
        }
        println!("{}: command not found", input.trim());
        input.clear();
        print!("$ ");
        io::stdout().flush().unwrap();
    }
}
