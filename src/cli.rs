use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "fSearch")]
#[command(about = "A lightweight CLI tool to search and manage files", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Change the current working directory
    Cd {
        /// Path to the directory
        path: String,
    },
    /// List files and folders in the current directory
    Ls,
    /// Search for a file or directory
    Find {
        /// Search for a file by name
        #[arg(short = 'f', long = "file")]
        file: Option<String>,
        /// Search for a directory by name
        #[arg(short = 'd', long = "dir")]
        dir: Option<String>,
        /// Search with a regex pattern
        #[arg(short = 'r', long = "regex")]
        regex: Option<String>,
        /// Search by file extension
        #[arg(short = 'e', long = "ext")]
        ext: Option<String>,
    },
    /// Filter directories
    Filter {
        /// Filter by type (empty, nonempty, hidden)
        #[arg(short = 't', long = "type")]
        filter_type: String,
    },
    /// Export found paths to a file
    Export {
        /// Target file path
        #[arg(short = 'o', long = "output")]
        file: String,
    },
    /// Exit the application
    Exit,
    /// Quit the application
    Quit,
}
