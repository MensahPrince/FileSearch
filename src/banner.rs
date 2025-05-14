use colored::*;

pub fn print_banner() {
    let wide_line = ".';:ccccccccccccccccc:;,.".bright_cyan().bold();
    let short_line = ".';:ccccc:;,.".bright_cyan().bold();

    println!("{}          {}", wide_line, "Simple CLI File Search Tool".bright_white());
    println!("{}          {}", wide_line, "    Made with Rust ❤️".bright_red());

    let pattern = [
        short_line.clone(),
        short_line.clone(),
        wide_line.clone(),
        wide_line.clone(),
        short_line.clone(),
        short_line.clone(),
        short_line.clone(),
        short_line.clone(),
        short_line.clone(),
    ];

    for line in pattern.iter() {
        println!("{}", line);
    }
}
