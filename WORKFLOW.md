# 🧰 FileSearch CLI – Version 0.1.0

## Overview

A minimal shell-like CLI built in Rust that currently supports basic commands like `cd` and `ls`, with a parser system designed to scale easily as more commands are added.

---

## 📦 Key Imports

```rust
use std::env::{current_dir, set_current_dir}; // Handle working directories
use std::io::{stdin, stdout, Write};          // Handle user input/output
use walkdir::WalkDir;                          // For directory traversal
```

---

## 🔄 Core Command Loop – Pseudocode

```pseudocode
loop {
    display_current_prompt()         // Print current working directory as shell prompt
    user_input = read_input_line()   // Read user input from stdin

    if user_input is empty:
        continue

    if user_input is "exit" or "quit":
        break                        // Terminate application

    command = Parser::parse(user_input)

    match command:
        Command::Cd(path) => change_directory(path)
        Command::Ls       => list_directory_contents()
        _                 => print "Unknown command"
}
```

---

## 🧩 Parser Design

The parser interprets user input by separating the keyword from arguments and turning it into structured `Command` enums. This makes command handling modular and extendable.

### ✅ Supported Commands

- `cd <path>` – Change the current working directory
- `ls` – List files and folders in the current directory
- `exit` or `quit` – Exit the CLI

---

## 🧠 Parser Pseudocode

```pseudocode
enum Command {
    Cd(path)
    Ls
    // Additional commands can be added here
}

struct Parser {
    // Optional fields for config, state, or current dir
}

impl Parser {
    function new() -> Parser {
        return Parser {
            // Initialize internal state if needed
        }
    }

    function parse(input_string) -> Command {
        words = split input_string by whitespace

        if words[0] == "cd" and length(words) > 1:
            return Command::Cd(words[1])
        else if words[0] == "ls":
            return Command::Ls
        else:
            return error("Unrecognized command")
    }
}
```

---

## 🗃️ Project Structure

```
FileSearch/
├── src/
│   ├── main.rs        // Main CLI logic
│   └── parser.rs      // Command parser module
├── Cargo.toml         // Rust project metadata
└── README.md          // This file
```

---

## 🚀 Future Work

- Add support for more shell commands (e.g., `mkdir`, `rm`, `touch`)
- Implement command history
- Add autocompletion
- Improve error handling with custom error types

---

## 📄 License

This project is licensed under the MIT License.
