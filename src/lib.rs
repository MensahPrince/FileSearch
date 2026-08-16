pub mod cli;
pub mod parser;
pub mod search;
pub mod utils;

use crate::cli::{Cli, Commands};
use crate::parser::Parser;
use crate::search::{export_dirs, filter_by, find_by_cli};
use crate::utils::banner::print_banner;
use crate::utils::prompt::print_prompt;
use colored::Colorize;
use std::io::{self};
use std::path::PathBuf;

pub fn run() {
    let mut found_paths: Vec<PathBuf> = Vec::new();
    print_banner();
    let parser = Parser::new();

    loop {
        print_prompt();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            eprintln!("fsearch: failed to read input");
            break;
        }

        let trimmed = input.trim();
        if trimmed.is_empty() {
            continue;
        }

        match parser.parse(trimmed) {
            Ok(cli) => match cli.command {
                Commands::Cd { path } => {
                    if let Err(error) = std::env::set_current_dir(&path) {
                        println!("fsearch: cd: {}: {}", path, error);
                    }
                }
                Commands::Ls => match std::fs::read_dir(".") {
                    Ok(entries) => {
                        for entry in entries.flatten() {
                            let name = entry.file_name();
                            let display_name = name.to_string_lossy();

                            if entry.metadata().map(|meta| meta.is_dir()).unwrap_or(false) {
                                println!("{}", display_name.green().bold());
                            } else if display_name.starts_with('.') {
                                println!("{}", display_name.red().bold());
                            } else {
                                println!("{}", display_name);
                            }
                        }
                    }
                    Err(error) => println!("fsearch: ls error: {}", error),
                },
                Commands::Find { file, dir, regex, ext } => {
                    find_by_cli(file, dir, regex, ext, &mut found_paths);
                }
                Commands::Filter { filter_type } => {
                    filter_by(&filter_type);
                }
                Commands::Export { file } => {
                    export_dirs(&found_paths, &file);
                }
                Commands::Exit | Commands::Quit => {
                    println!("Exiting...");
                    break;
                }
            },
            Err(error) => {
                let _ = error.print();
                println!();
            }
        }
    }
}

pub fn parse_cli(input: &str) -> Result<Cli, clap::Error> {
    Parser::new().parse(input)
}
