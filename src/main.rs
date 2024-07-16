mod constants;
mod file;
mod git;
mod setup;
mod utils;

use crate::setup::setup_go_project;
use crate::setup::setup_rust_project;
use crate::utils::prompt_database_selection;

use clap::{Command, Arg};
use colored::*;
use console::{style, Term};
use dialoguer::{theme::ColorfulTheme, Input, Select};
use dirs::desktop_dir;
use dotenv::dotenv;

use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let term = Term::stdout();
    print_banner();

    let default_path = desktop_dir()
        .expect("could not find desktop directory")
        .to_str()
        .expect("Desktop path is not valid UTF-8")
        .to_string();

    let matches = Command::new("Genesis")
        .version("1.0.0")
        .author("Thembinkosi Mkhonta")
        .about("Sets up starter projects for Go or Rust")
        .arg(
            Arg::new("language")
                .short('l')
                .long("language")
                .value_name("LANGUAGE")
                .help("Sets the project language (go or rust)")
        )
        .arg(
            Arg::new("path")
                .short('p')
                .long("path")
                .value_name("PATH")
                .help("Sets the path where the project will be cloned")
                .default_value("~/Desktop")  // Use a default string literal
        )
        .get_matches();

    let language = match matches.get_one::<String>("language").map(|s| s.as_str()) {
        Some(lang) => lang.to_string(),
        None => prompt_step(&term, "Choose your project language:", || {
            let options = &["Go", "Rust"];
            let selection = Select::with_theme(&ColorfulTheme::default())
                .items(&options[..])
                .default(0)
                .interact_on(&term)?;
            Ok(options[selection].to_lowercase())
        })?,
    };

    let project_name = prompt_step(&term, "Enter your project name:", || {
        Ok(Input::<String>::new().interact_text()?)
    })?;

    // Parse the path after command-line arguments are processed
    let path = matches
        .get_one::<String>("path")
        .map(|s| PathBuf::from(shellexpand::tilde(s).into_owned()))
        .unwrap_or_else(|| PathBuf::from(default_path));

    match language.as_str() {
        "go" => {
            let module_name = prompt_step(&term, "Enter your Go module name:", || {
                Ok(Input::<String>::new()
                    .with_prompt("(e.g., github.com/username/project)")
                    .interact_text()?)
            })?;
            let database = prompt_database_selection(&term)?;
            setup_go_project(path.to_str().unwrap(), &project_name, &module_name, &database)?
        }
        "rust" => setup_rust_project(path.to_str().unwrap(), &project_name)?,
        _ => println!(
            "{}",
            style(format!("Unsupported language: {}", language)).red()
        ),
    }

    Ok(())
}
fn prompt_step<T>(
    term: &Term,
    prompt: &str,
    input_fn: impl FnOnce() -> Result<T, Box<dyn std::error::Error>>,
) -> Result<T, Box<dyn std::error::Error>> {
    term.clear_last_lines(2)?;
    println!("{}", style(prompt).cyan().bold());
    let result = input_fn()?;
    term.clear_last_lines(1)?;
    println!();
    println!();

    println!("{} {}", style("✓").green().bold(), style(prompt).dim());
    Ok(result)
}

fn print_banner() {
    println!("{}", "\n".repeat(2));
    println!(
        "{}",
        r#"   ______                      _     "#.bright_cyan()
    );
    println!(
        "{}",
        r#"  / ____/___  ____  ___  _____(_)____"#.bright_cyan()
    );
    println!(
        "{}",
        r#" / / __/ __ \/ __ \/ _ \/ ___/ / ___/"#.bright_cyan()
    );
    println!(
        "{}",
        r#"/ /_/ / /_/ / / / /  __(__  ) (__  ) "#.bright_cyan()
    );
    println!(
        "{}",
        r#"\____/\____/_/ /_/\___/____/_/____/  "#.bright_cyan()
    );
    println!("{}", "\n".repeat(2));
    println!(
        "{}",
        "Welcome to Genesis - Your Project Starter!"
            .bright_green()
            .bold()
    );
    println!(
        "{}",
        "------------------------------------------".bright_green()
    );
    println!("{}", "\n".repeat(3));
}
