// Define supported commands
#[derive(Debug)]
pub enum Command {
    Cd(String),
    Ls,
    //  More commands would be added here as needed,
    Invalid(String), // Gracefully handle unrecognized input
}

// Parser struct
pub struct Parser;

impl Parser {
    // Create a new parser instance
    pub fn new() -> Self {
        Parser
    }

    // Parse a string input and return a Command
    pub fn parse(&self, input: &str) -> Command {
        let trimmed = input.trim();
        let tokens: Vec<&str> = trimmed.split_whitespace().collect();

        match tokens.as_slice() {
            ["cd", path] => Command::Cd(path.to_string()),
            ["ls"] => Command::Ls,
            [cmd, ..] => Command::Invalid(cmd.to_string()),
            [] => Command::Invalid("".to_string()),
        }
    }
}
