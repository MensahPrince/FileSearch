use colored::Colorize;
use std::env;
use std::io::{self, Write};

pub fn print_prompt() {
    let hostname = gethostname::gethostname().to_string_lossy().into_owned();
    let current_dir = env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("."));
    let dir_name = current_dir
        .file_name()
        .map(|name| name.to_string_lossy().into_owned())
        .unwrap_or_else(|| "/".to_string());

    print!(
        "{} ",
        format!("fsearch@{}:{}$", hostname, dir_name)
            .bright_blue()
            .bold()
    );
    io::stdout().flush().unwrap();
}
