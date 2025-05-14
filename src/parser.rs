// Define supported commands
#[derive(Debug)]
pub enum Command {
    Cd(String),
    Ls,
    FindDir(String),
    FindFile(String),
    Empty(()), // handle empty input
    Invalid(String), // handle unrecognized input
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

        if trimmed.is_empty() {
            return Command::Empty(());
        }
        
        let tokens: Vec<&str> = trimmed.split_whitespace().collect();

        match tokens.as_slice() {
            ["cd", path] => Command::Cd(path.to_string()),
            ["ls"] => Command::Ls,
            ["find", "-dir", name ] => Command::FindDir(name.to_string()),
            ["find", "-f", name] => Command::FindFile(name.to_string()),
            ["", ..] => Command::Empty(()),
            [] => Command::Invalid("".to_string()),
            [cmd, ..] => Command::Invalid(cmd.to_string()),
        }
    }
}
