use std::env::current_dir;
use std::io::{self, Write};

mod parser;
use parser::{Parser, Command};
use walkdir::WalkDir;

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
}

// Display current working directory prompt
fn curr_dir_rtn() {
    // Get the current working directory
    let curr_dir = current_dir().unwrap();
    print!("{} ", format!("fsearch@host:{}$", curr_dir.display()));
    io::stdout().flush().unwrap();
}

fn fnd_file(name: &str){
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
                if file_name == name {
                    println!("Found file: {}", entry.path().display());
                }
            }
        }
}
//A function to find a child directory in its parent dir.
fn fnd_dir(name: &str){
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
                }
            }
        }
}

fn no_cmd(_: ()) {
    /*
    This function is used to skip empty input
    */
}

fn main() {
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
                            println!("{}", entry.file_name().to_string_lossy());
                        }
                    }// If an error occurs while reading the directory:
                    Err(e) => {
                        // Print the error message
                        println!("fsearch: ls error: {}", e);
                    }
                }

            }
            Command::FindDir(name) => {
                fnd_dir(&name);
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
            
        }
    }
}
