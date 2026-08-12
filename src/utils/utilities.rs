mod banner;
mod cli;
mod parser;
use banner::print_banner;
use cli::Commands;
use colored::*;
use parser::Parser;
use regex::Regex;
use std::env::current_dir;
use std::fs::File;
use std::io::{self, Write};
use std::path::PathBuf;
use walkdir::WalkDir;

// Display current working directory prompt
fn curr_dir_rtn() {
    // Get the actual system hostname
    let hostname = gethostname::gethostname().to_string_lossy().into_owned();

    // Get the current working directory
    let curr_dir = current_dir().unwrap();

    // Get only the last component of the path (the current directory name)
    let dir_name = curr_dir
        .file_name()
        .map(|os_str| os_str.to_string_lossy().into_owned())
        .unwrap_or_else(|| "/".to_string());

    // Print the refactored prompt
    print!(
        "{} ",
        format!("fsearch@{}:{}$", hostname, dir_name)
            .bright_blue()
            .bold()
    );
    io::stdout().flush().unwrap();
}

fn find_by_cli(
    file: Option<String>,
    dir: Option<String>,
    regex: Option<String>,
    ext: Option<String>,
    found_paths: &mut Vec<PathBuf>,
) {
    if let Some(name) = file {
        let results = fnd_file(&name);
        found_paths.extend(results);
    }
    if let Some(name) = dir {
        fnd_dir(&name, found_paths);
    }
    if let Some(pattern) = regex {
        // Need to update find_by_regex to return paths or update found_paths
        find_by_regex(&pattern, found_paths);
    }
    if let Some(extension) = ext {
        find_ext(&extension, found_paths);
    }
}

fn fnd_file(name: &str) -> Vec<PathBuf> {
    //A path variable to hold the path of the current (parent) dir
    let curr_dir = std::env::current_dir().unwrap();
    let mut found: Vec<PathBuf> = Vec::new();

    //For loop: to loop through the children dirs of the parent dir (curr_dir)
    for entry in WalkDir::new(curr_dir)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_file())
    {
        if let Some(file_name) = entry.file_name().to_str()
            && file_name == name
        {
            println!("Found file: {}", entry.path().display());
            found.push(entry.path().to_path_buf());
        }
    }
    found
}
//A function to find a child directory in its parent dir.
fn fnd_dir(name: &str, found_paths: &mut Vec<PathBuf>) {
    //A path variable to hold the path of the current (parent) dir
    let curr_dir = std::env::current_dir().unwrap();

    //For loop: to loop through the children dirs of the parent dir (curr_dir)
    for entry in WalkDir::new(curr_dir)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_dir())
    {
        if let Some(dir_name) = entry.file_name().to_str() {
            if dir_name == name {
                println!("Found directory: {}", entry.path().display());
                found_paths.push(entry.path().to_path_buf());
            }
        }
    }
}

fn find_ext(ext: &str, found_paths: &mut Vec<PathBuf>) {
    //A path variable to hold the path of the current (parent) dir
    let curr_dir = std::env::current_dir().unwrap();

    //For loop: to loop through the children dirs of the parent dir (curr_dir)
    for entry in WalkDir::new(curr_dir)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_file())
    {
        if let Some(file_name) = entry.file_name().to_str()
            && file_name.ends_with(ext)
        {
            println!(
                "Found file with extension {}: {}",
                ext,
                entry.path().display()
            );
            found_paths.push(entry.path().to_path_buf());
        }
    }
}

fn find_by_regex(pattern: &str, found_paths: &mut Vec<PathBuf>) {
    let re = Regex::new(pattern).expect("Invalid regex pattern");

    for entry in WalkDir::new(".")
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_file())
    {
        let file_name = entry.file_name().to_string_lossy();
        if re.is_match(&file_name) {
            println!("Matched: {}", entry.path().display());
            found_paths.push(entry.path().to_path_buf());
        }
    }
}

pub fn export_dirs(found_paths: &Vec<PathBuf>, file_path: &str) {
    match File::create(file_path) {
        Ok(mut exfile) => {
            for path in found_paths {
                if let Err(e) = writeln!(exfile, "{}", path.display()) {
                    println!("Failed to write to file: {}", e);
                    return;
                }
            }
            println!("Successfully exported paths to {}", file_path);
        }
        Err(e) => {
            println!("Failed to create file '{}': {}", file_path, e);
        }
    }
}

fn filter_by(filter_type: &str) {
    match filter_type {
        "empty" => {
            for entry in WalkDir::new(".")
                .into_iter()
                .filter_map(Result::ok)
                .filter(|e| e.file_type().is_dir())
            {
                if std::fs::read_dir(entry.path())
                    .map(|mut i| i.next().is_none())
                    .unwrap_or(false)
                {
                    println!("Empty directory: {}", entry.path().display());
                }
            }
        }
        "nonempty" => {
            for entry in WalkDir::new(".")
                .into_iter()
                .filter_map(Result::ok)
                .filter(|e| e.file_type().is_dir())
            {
                if std::fs::read_dir(entry.path())
                    .map(|mut i| i.next().is_some())
                    .unwrap_or(false)
                {
                    println!("Non-empty directory: {}", entry.path().display());
                }
            }
        }
        "hidden" => {
            for entry in WalkDir::new(".").into_iter().filter_map(Result::ok) {
                if let Some(name) = entry.file_name().to_str() {
                    if name.starts_with('.') {
                        println!("Hidden: {}", entry.path().display());
                    }
                }
            }
        }
        _ => {
            println!("Unknown filter type: '{}'", filter_type);
        }
    }
}

fn main() {
    let mut found_paths: Vec<PathBuf> = Vec::new();
    //Print the banner
    print_banner();
    //Parser Instance & initialization
    let parser = Parser::new();

    //Main working loop.
    loop {
        //function to display the current working directory
        curr_dir_rtn();

        //A mutable to store user input
        let mut input_line = String::new();
        //Read user Input
        io::stdin()
            .read_line(&mut input_line)
            .expect("Failed to read line");
        //Trim the input of whitespaces
        let trimmed_input = input_line.trim();

        if trimmed_input.is_empty() {
            continue;
        }

        // Pass the trimmed input to the parser and match the result
        match parser.parse(trimmed_input) {
            Ok(cli) => match cli.command {
                Commands::Cd { path } => {
                    if let Err(e) = std::env::set_current_dir(&path) {
                        println!("fsearch: cd: {}: {}", path, e);
                    }
                }
                Commands::Ls => match std::fs::read_dir(".") {
                    Ok(entries) => {
                        for entry in entries.flatten() {
                            let file_name = entry.file_name();
                            let display_name = file_name.to_string_lossy();
                            if entry.metadata().map(|m| m.is_dir()).unwrap_or(false) {
                                println!("{}", display_name.green().bold());
                            } else if display_name.starts_with('.') {
                                println!("{}", display_name.red().bold());
                            } else {
                                println!("{}", display_name);
                            }
                        }
                    }
                    Err(e) => println!("fsearch: ls error: {}", e),
                },
                Commands::Find {
                    file,
                    dir,
                    regex,
                    ext,
                } => {
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
            Err(e) => {
                // Use clap's built-in error printing which handles color and help/version properly
                let _ = e.print();
                println!(); // Add a newline after clap's output
            }
        }
    }
}
