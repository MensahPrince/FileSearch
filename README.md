## N.B.: This project would be released in phases, each with a planned set of achieved features.

# FSearch
## Phase 1
## Features for v0.1.0

- Surf directories ✅
- Recursively walks through a directory ✅
- Prints out the full path of each file and subdirectory ✅
- Basic error handling for unreadable files or folders ✅

## Usage for version 0.1.0

1. Clone the repository or copy the source code.

2. Add `walkdir`,`colored` and `regex` to your `Cargo.toml`:

```toml
[dependencies]
walkdir = "2.5.0"
colored = "3"
regex = "1.11.1
```
Build the project: cargo build. 

Run the project: cargo run.

## Phase 2: CLI Enhancements (v0.2.0)

- Add support for filtering by file extension ✅
- Add regex-based search for file names ✅
- Export results to a text file ✅
- Add colored output for files/folders ✅

## Phase 3: CLI UX & Config (v0.3.0)
- Add interactive mode (optional prompts for path, filters)  
- Add a config file or command-line flags for customization  
- Improved error reporting and logging
