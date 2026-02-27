use crate::cli::Cli;
use clap::Parser as ClapParser;

// Parser struct
pub struct Parser;

impl Parser {
    // Create a new parser instance
    pub fn new() -> Self {
        Parser
    }

    // Parse a string input and return a Result<Cli, clap::Error>
    pub fn parse(&self, input: &str) -> Result<Cli, clap::Error> {
        let trimmed = input.trim();

        // Map empty input to a special case or just return error
        if trimmed.is_empty() {
            // We can return a custom error or just use clap's help
            return ClapParser::try_parse_from(vec!["fsearch", "--help"]);
        }

        let mut words = vec!["fsearch"];
        words.extend(trimmed.split_whitespace());

        ClapParser::try_parse_from(words)
    }
}
