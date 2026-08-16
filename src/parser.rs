use crate::cli::Cli;
use clap::Parser as ClapParser;

pub struct Parser;

impl Parser {
    pub fn new() -> Self {
        Self
    }

    pub fn parse(&self, input: &str) -> Result<Cli, clap::Error> {
        let trimmed = input.trim();

        if trimmed.is_empty() {
            return ClapParser::try_parse_from(["fsearch", "--help"]);
        }

        let mut words = vec!["fsearch".to_string()];
        words.extend(trimmed.split_whitespace().map(str::to_owned));

        ClapParser::try_parse_from(words)
    }
}
