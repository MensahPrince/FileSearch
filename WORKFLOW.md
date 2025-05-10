## 🧰 Workflow for Version 0.1.0 (Initial CLI Version)

### 📦 Key Imports

```rust
use std::env::set_current_dir;           // Change the current working directory
use std::env::current_dir;               // Get the current working directory
use std::io::stdin;                      // Read terminal input
use walkdir::WalkDir;                    // Traverse directories and their descendants
```

---

### 🧠 Pseudocode: Core Command Loop

```pseudocode
loop {
    print current directory
    read input
    match command:
        - "cd <path>": change directory         // Change the working directory
        - "ls": list files in current dir       // Display contents using WalkDir
        - "exit" or "quit": break               // Exit the application
        - invalid: print error                  // Handle unknown or malformed input
}
```

# PARSER
### This is a parser to take in user commands with arguments, break them down before fsearch executes them 
### I find this method more convenient than using other conventional methods like If-else statements or match cases.

```pseudocode 

enum commands{
    cd (path)
    ls 
    //Limited to two commands for now. 
}

struct parser{
    curr_dir 
    config_options 
}

//Implementations for the parser
impl parser{
    //constructor to initialize the parser
    function new() -> parser{
        return parser{
            curr_dir = get_curr_dir();
            config_options = default_config()
        }
    }
    
}
```