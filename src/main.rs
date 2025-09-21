mod command;
#[allow(unused_imports)]
use std::io::{self, Write};
use crate::command::Command;

fn main() {
    // Uncomment this block to pass the first stage
    loop {
        let stdin = io::stdin();
        print!("$ ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();
        let cmd = Command::from_input(&input);
        match cmd {
            Command::Exit => break,
            Command::Echo { display_string } => println!("{display_string}"),
            Command::NotFound => println!("{:}: command not found", input.trim()),
        }
    }
}
