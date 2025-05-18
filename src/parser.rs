// Define supported commands
#[derive(Debug)]
pub enum Command {
    Cd(String),
    Ls,
    FindBy(String, String),
    //FindFile(String),
    //FindExt(String),
    //FindRegex(String), 
    FilterBy(String), 
    Empty(()), // handle empty input
    Invalid(String), // handle unrecognized input
    Export(String),//Allows export of found directories to a file
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
            ["find", handler, name ] => Command::FindBy(handler.to_string(), name.to_string()),
            //["find", "-f", name] => Command::FindFile(name.to_string()),
            //["find", "-ext", ext] => Command::FindExt(ext.to_string()),
           // ["find", "-regex", regex] => Command::FindRegex(regex.to_string()), 
            ["export", "->", file] => Command::Export(file.to_string()),
            ["filter", "-type", "-dir", filter_type] => Command::FilterBy(filter_type.to_string()),
            ["", ..] => Command::Empty(()),
            [] => Command::Invalid("".to_string()),
            [cmd, ..] => Command::Invalid(cmd.to_string()),
        }
    }
}
