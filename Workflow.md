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
