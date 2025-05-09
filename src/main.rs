use std::env::current_dir;
use std::io::{self, Write};

fn run() {
    //function to display the help menu which would be called when the user types "help" or "-h"
    fn help() {
        println!("Available Commands:");
        println!("  help    or  -h    Show this help message");
        println!("  exit       Exit the application");
        // Future commands can be listed here as you add them
    }

    //A function to return the current working directory
    fn curr_dir_rtn(){
        let curr_dir = current_dir().unwrap(); // Get the current working directory
        print!("{} ", format!("fsearch@host:{}$ ", curr_dir.display())); // Prompt
        io::stdout().flush().unwrap(); // Flush the output so it appears before input
    }

    loop {
        curr_dir_rtn();
        let mut read_dir = String::new(); // Reset input string every loop
        io::stdin()
            .read_line(&mut read_dir)
            .expect("Failed to read line");

        let input = read_dir.trim(); // Trim whitespace
        //Match the input
        match input {
            "exit" => {
                println!("Exiting...");
                break; // Exit the loop
            }
            "help" | "-h"=> {
                help();
            }
            "" => {
                // Do nothing on empty input
            }
            _ => {
                println!("fsearch: Unknown command: {}", input);
            }
        }
    }
}



fn main() {
    run();
}
