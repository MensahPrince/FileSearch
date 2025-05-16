use colored::*;

pub fn print_banner() {
    let wide_line = ".';:ccccccccccccccccc:;,.";
    let short_line = ".';:ccccc:;,.";
    
    let wide = wide_line.bright_cyan().bold();
    let short = short_line.bright_cyan().bold();
    
    let title = "Simple CLI File Search Tool".bright_white().bold();
    let author = "Made with Rust ❤️".bright_red().bold();
    let version = "v0.2.0".bright_yellow().bold();

    // Create a banner block
    let banner = vec![
        wide.clone(),
        wide.clone(),
        short.clone(),
        short.clone(),
        wide.clone(),
        wide.clone(),
        format!("{}                        {}", short, title).bright_cyan().bold(),
        format!("{}                        {}", short, author).bright_cyan().bold(),
        format!("{}                        {}", short, version).bright_cyan().bold(),
        short.clone(),
        short.clone(),
        short.clone(),
        short.clone(),
    ];

    // Print the banner
    for line in banner {
        println!("{}", line);
    }
}
