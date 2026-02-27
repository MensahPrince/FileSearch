use colored::*;
use figlet_rs::FIGfont;

pub fn print_banner() {
    let standard_font = FIGfont::standard().unwrap();
    let figure = standard_font.convert("fSearch");

    let title = "Lightweight CLI File Search Tool".bright_white().bold();
    let author = "Made with Rust ❤️".bright_red().bold();
    let version = "v0.5.0".bright_yellow().bold();

    if let Some(fig) = figure {
        // Print the figlet art in cyan
        println!("{}", fig.to_string().bright_cyan().bold());
    }

    println!("  {}", title);
    println!("  {} | {}", author, version);
    println!("{}", "=".repeat(50).bright_black());
    println!();
}
