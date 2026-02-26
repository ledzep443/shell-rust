
pub(crate) enum Command {
    Exit,
    Echo { display_string: String },
    TypeOf { command: String },
    NotFound
}

impl Command {
    pub fn from_input(s: &str) -> Self {
        let input = s.trim().to_ascii_lowercase();
        match input.trim().split(' ').collect::<Vec<&str>>().first().unwrap() {
            &"exit" => Self::Exit,
            &"echo" => Self::Echo { display_string: input["echo ".len()..].to_string() },
            &"type" => Self::TypeOf { command: input["type ".len()..].to_string() },
            _ => Self::NotFound
        }
    }
}