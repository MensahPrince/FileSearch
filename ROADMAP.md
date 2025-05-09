RoadMap,

This roadmap outlines the planned features and development milestones for the WalkDir-based file lister tool.

---

## Phase 1: Initial CLI Version (v0.1.0)
- Read files and directories using `walkdir`  
- Print paths to terminal  
- Basic error handling for inaccessible entries  
- Surf directories interactively (basic listing navigation)

---

## Phase 2: CLI Enhancements (v0.2.0)
- Add support for filtering by file extension  
- Add regex-based search for file names  
- Export results to a text file  
- Add colored output for files/folders (via `colored` or `console` crate)

---

## Phase 3: CLI UX & Config (v0.3.0)
- Add interactive mode (optional prompts for path, filters)  
- Add a config file or command-line flags for customization  
- Improved error reporting and logging

---

## Phase 4: GUI Version (v1.0.0)
- Build cross-platform GUI using `egui`, `iced`, or `druid`  
- File explorer-style interface with collapsible tree view  
- Live search and pattern filtering  
- Export buttons (save results, copy path, etc.)  
- Theme support (light/dark)

---

## Future Ideas
- Bookmark commonly used paths  
- Add plugin support for custom actions on files  
- File preview panel in GUI  
- Terminal and GUI modes in one binary (`--gui` flag)

---

> Contributions and suggestions are welcome!  
> Open an issue or discussion to help shape the future of this project.
