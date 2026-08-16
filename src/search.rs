use regex::Regex;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

fn current_dir_root() -> PathBuf {
    std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."))
}

fn walk_files(root: &Path) -> impl Iterator<Item = walkdir::DirEntry> {
    WalkDir::new(root)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|entry| entry.file_type().is_file())
}

fn walk_dirs(root: &Path) -> impl Iterator<Item = walkdir::DirEntry> {
    WalkDir::new(root)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|entry| entry.file_type().is_dir())
}

pub fn find_by_cli(
    file: Option<String>,
    dir: Option<String>,
    regex: Option<String>,
    ext: Option<String>,
    found_paths: &mut Vec<PathBuf>,
) {
    if let Some(name) = file {
        found_paths.extend(find_file(&name));
    }

    if let Some(name) = dir {
        find_dir(&name, found_paths);
    }

    if let Some(pattern) = regex {
        find_by_regex(&pattern, found_paths);
    }

    if let Some(extension) = ext {
        find_ext(&extension, found_paths);
    }
}

pub fn find_file(name: &str) -> Vec<PathBuf> {
    let root = current_dir_root();
    let mut found = Vec::new();

    for entry in walk_files(&root) {
        if let Some(file_name) = entry.file_name().to_str()
            && file_name == name
        {
            println!("Found file: {}", entry.path().display());
            found.push(entry.path().to_path_buf());
        }
    }

    found
}

pub fn find_dir(name: &str, found_paths: &mut Vec<PathBuf>) {
    let root = current_dir_root();

    for entry in walk_dirs(&root) {
        if let Some(dir_name) = entry.file_name().to_str()
            && dir_name == name
        {
            println!("Found directory: {}", entry.path().display());
            found_paths.push(entry.path().to_path_buf());
        }
    }
}

pub fn find_ext(ext: &str, found_paths: &mut Vec<PathBuf>) {
    let root = current_dir_root();

    for entry in walk_files(&root) {
        if let Some(file_name) = entry.file_name().to_str()
            && file_name.ends_with(ext)
        {
            println!("Found file with extension {}: {}", ext, entry.path().display());
            found_paths.push(entry.path().to_path_buf());
        }
    }
}

pub fn find_by_regex(pattern: &str, found_paths: &mut Vec<PathBuf>) {
    let re = Regex::new(pattern).unwrap_or_else(|_| panic!("Invalid regex pattern: {}", pattern));
    let root = current_dir_root();

    for entry in walk_files(&root) {
        let file_name = entry.file_name().to_string_lossy();
        if re.is_match(&file_name) {
            println!("Matched: {}", entry.path().display());
            found_paths.push(entry.path().to_path_buf());
        }
    }
}

pub fn export_dirs(found_paths: &[PathBuf], file_path: &str) {
    match File::create(file_path) {
        Ok(mut output) => {
            for path in found_paths {
                if let Err(error) = writeln!(output, "{}", path.display()) {
                    println!("Failed to write to file: {}", error);
                    return;
                }
            }
            println!("Successfully exported paths to {}", file_path);
        }
        Err(error) => {
            println!("Failed to create file '{}': {}", file_path, error);
        }
    }
}

pub fn filter_by(filter_type: &str) {
    match filter_type {
        "empty" => {
            for entry in walk_dirs(Path::new(".")) {
                if std::fs::read_dir(entry.path())
                    .map(|mut children| children.next().is_none())
                    .unwrap_or(false)
                {
                    println!("Empty directory: {}", entry.path().display());
                }
            }
        }
        "nonempty" => {
            for entry in walk_dirs(Path::new(".")) {
                if std::fs::read_dir(entry.path())
                    .map(|mut children| children.next().is_some())
                    .unwrap_or(false)
                {
                    println!("Non-empty directory: {}", entry.path().display());
                }
            }
        }
        "hidden" => {
            for entry in WalkDir::new(".")
                .into_iter()
                .filter_map(Result::ok)
            {
                if let Some(name) = entry.file_name().to_str()
                    && name.starts_with('.')
                {
                    println!("Hidden: {}", entry.path().display());
                }
            }
        }
        _ => {
            println!("Unknown filter type: '{}'", filter_type);
        }
    }
}
