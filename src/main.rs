mod parser;
mod banner;
use std::env::current_dir;
use std::io::{self, Write};
use parser::{Parser, Command};
use walkdir::WalkDir;
use regex::Regex;
use banner::print_banner;
use colored::*;
use std::path::PathBuf;
use std::fs::File;


// Display help menu
fn help() {
    println!("Available Commands:");
    println!("  help    or  -h  ----  Show this help message");
    println!("  exit       ----  Exit the application");
    println!("  cd <path>  ----  Change current directory");
    println!("  ls         ----  List contents of the current directory");
    println!("  find       ----  Search for a file or directory in any given path");
    println!("  -dir <name>  ----  Used together with \"find\" to search for a directory in the current directory");
    println!("  -f <name>  ----  Used together with \"find\" to search for a file in the current directory");
    println!("  -regex <pattern>  ----  Used together with \"find\" to search for files matching a regex pattern");
    println!("  -ext <extension>  ----  Used together with \"find\" to search for files with a specific extension in the current directory");
    println!("  export -> <file>  ----  Export found directories to a file");
}

// Display current working directory prompt
fn curr_dir_rtn() {
    // Get the current working directory
    let curr_dir = current_dir().unwrap();
    print!("{} ", format!("fsearch@host:{}$", curr_dir.display()));
    io::stdout().flush().unwrap();
}

fn fnd_file(name: &str) -> Vec<PathBuf>{
    //A path variable to hold the path of the current (parent) dir
    let curr_dir = std::env::current_dir().unwrap();
    let  mut found: Vec<PathBuf> = Vec::new();

    //For loop: to loop through the children dirs of the parent dir (curr_dir)
    for entry in WalkDir::new(curr_dir)
        //Creates an iterator from a value.
        .into_iter()
        //Creates an iterator that both filters and maps.
        .filter_map(Result::ok)
        //Creates an iterator which uses a closure to determine if an element should be yielded.
        .filter(|e| e.file_type().is_file())
        {
            if let Some(file_name) = entry.file_name().to_str(){
                if file_name == name {
                    println!("Found file: {}", entry.path().display());
                    found.push(entry.path().to_path_buf());
                }
            }
        }
    found
}
//A function to find a child directory in its parent dir.
fn fnd_dir(name: &str, found_paths: &mut Vec<PathBuf>){
    //A path variable to hold the path of the current (parent) dir
    let curr_dir = std::env::current_dir().unwrap();

    //For loop: to loop through the children dirs of the parent dir (curr_dir)
    for entry in WalkDir::new(curr_dir)
        //Creates an iterator from a value.
        .into_iter()
        //Creates an iterator that both filters and maps.
        .filter_map(Result::ok)
        //Creates an iterator which uses a closure to determine if an element should be yielded.
        .filter(|e| e.file_type().is_dir())
        {
            if let Some(dir_name) = entry.file_name().to_str(){
                if dir_name == name {
                    println!("Found directory: {}", entry.path().display());
                    found_paths.push(entry.path().to_path_buf());
                }
            }
        }
}

fn no_cmd(_: ()) {
    /*
    This function is used to skip empty input
    */
}

fn find_ext(ext: &str, found_paths: &mut Vec<PathBuf>) {
    //A path variable to hold the path of the current (parent) dir
    let curr_dir = std::env::current_dir().unwrap();

    //For loop: to loop through the children dirs of the parent dir (curr_dir)
    for entry in WalkDir::new(curr_dir)
        //Creates an iterator from a value.
        .into_iter()
        //Creates an iterator that both filters and maps.
        .filter_map(Result::ok)
        //Creates an iterator which uses a closure to determine if an element should be yielded.
        .filter(|e| e.file_type().is_file())
        {
            if let Some(file_name) = entry.file_name().to_str(){
                if file_name.ends_with(ext) {
                    println!("Found file with extension {}: {}", ext, entry.path().display());
                    found_paths.push(entry.path().to_path_buf());
                }
            }
        }
}


fn find_by_regex(pattern: &str) {
    let re = Regex::new(pattern).expect("Invalid regex pattern");

    for entry in WalkDir::new(".")
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_file())
    {
        let file_name = entry.file_name().to_string_lossy();
        if re.is_match(&file_name) {
            println!("Matched: {}", entry.path().display());
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
        io::stdin().read_line(&mut input_line).expect("Failed to read line");
        //Trim the input of whitespaces
        let trimmed_input = input_line.trim();

        //Check if the trimmed user input is exit, if so break the loop.
        if trimmed_input == "exit" {
            println!("Exiting...");
            break;
        }

        //If the user input is help  or -h, show the help menu
        if trimmed_input == "help" || trimmed_input == "-h" {
            help();
            continue;
        }

        //pass the trimmed input to the parser and match the result to the Command enum 
        //See parser.rs for Command enum and parser implementation
        match parser.parse(trimmed_input) {
            //Match the command; if trimmed input is cd <path>, change the directory to the desired path
            Command::Cd(path) => {
                // If the path is invalid:
                if let Err(e) = std::env::set_current_dir(&path) {
                    // Print the error message
                    println!("fsearch: cd: {}: {}", path, e);
                }
            }//Match the command; if trimmed input is ls, list the contents of the current directory
            Command::Ls => {
                // Use the read_dir function to get the directory entries
                match std::fs::read_dir(".") {
                    Ok(entries) => {
                        // Iterate over the entries in the directory and print their names
                        // Flatten(): Creates an iterator that flattens nested structure.
                        for entry in entries.flatten() {
                            // Print the file name of each entry
                            // to_string_lossy(): Converts the file name to a string, replacing invalid UTF-8 sequences with U+FFFD.
                            // This is useful for displaying file names that may contain non-UTF-8 characters.
                            // file_name(): Returns the name of the file as OsString.
                            // See documentation for more details.
                            let file_name = entry.file_name();
                            let dir_color = file_name.to_string_lossy();
                            //conditional formatting: green if the entry is a directory, otherwise default color
                            if entry.metadata().unwrap().is_dir(){
                                println!("{}", dir_color.green().bold());
                            }else{
                                println!("{}", dir_color);
                            }
                        }
                    }// If an error occurs while reading the directory:
                    Err(e) => {
                        // Print the error message
                        println!("fsearch: ls error: {}", e);
                    }
                }

            }
            Command::FindDir(name) => {
                fnd_dir(&name, &mut found_paths);
            }
            Command::Empty(_) => {
                //Do nothing if the input is empty
                no_cmd(());
            }
            //Match the command; if trimmed input is invalid, print an error message
            // This handles any command that doesn't match the defined commands
            Command::Invalid(cmd) => {
                // Print an error message for the unrecognized command
                println!("fsearch: Unknown command '{}'", cmd);
            }
            Command::FindFile(name) => {
                fnd_file(&name);
            }
            Command::FindExt(ext) => {
                find_ext(&ext, &mut found_paths);
            } 
            Command::Export(file) =>{
                export_dirs(&found_paths, &file);
            }
            Command::FindRegex(regex) => {
                find_by_regex(&regex);
            }

        }
    }
}
