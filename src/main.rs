use clap::{App, Arg};
use colored::*;
use console::{style, Term};
use dialoguer::{theme::ColorfulTheme, Select};
use dotenv::dotenv;
use git2::{build::RepoBuilder, Cred, FetchOptions, Progress, RemoteCallbacks, Repository};
use indicatif::{ProgressBar, ProgressStyle};
use std::cell::RefCell;
use std::path::Path;
use std::rc::Rc;

use std::process::Command;

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
        "---------------------------------------".bright_green()
    );
    println!("{}", "\n");
}

fn get_language_selection() -> Result<String, Box<dyn std::error::Error>> {
    let options = &["Go", "Rust"];

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Choose your project language")
        .default(0)
        .items(&options[..])
        .interact_on_opt(&Term::stderr())?;

    match selection {
        Some(index) => Ok(options[index].to_lowercase()),
        None => Err("No language selected".into()),
    }
}

fn clone_repo(url: &str, path: &str) -> Result<Repository, git2::Error> {
    // let token = env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN not set");
    let token = "";

    // Create a progress bar
    let pb = Rc::new(RefCell::new(ProgressBar::new(100)));
    pb.borrow_mut().set_style(
        ProgressStyle::default_bar()
            .template(
                "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos:>7}/{len:7} {msg}",
            )
            .progress_chars("#>-"),
    );

    // Clone the Rc<RefCell<ProgressBar>> for the closure
    let pb_clone = pb.clone();

    let mut callbacks = RemoteCallbacks::new();
    callbacks.credentials(move |_url, _username_from_url, _allowed_types| {
        Cred::userpass_plaintext("git", &token)
    });

    callbacks.transfer_progress(move |stats: Progress| {
        let pb = pb_clone.borrow_mut();
        if stats.received_objects() == stats.total_objects() {
            pb.set_message("Resolving deltas...");
        } else if stats.total_objects() > 0 {
            pb.set_message("Receiving objects...");
        } else {
            pb.set_message("Preparing...");
        }

        let progress = if stats.total_objects() > 0 {
            (100 * stats.received_objects() / stats.total_objects()) as u64
        } else {
            0
        };
        pb.set_position(progress);
        true
    });

    let mut fetch_options = FetchOptions::new();
    fetch_options.remote_callbacks(callbacks);

    let mut builder = RepoBuilder::new();
    builder.fetch_options(fetch_options);

    let result = builder.clone(url, Path::new(path));

    pb.borrow_mut().finish_with_message("Done!");

    result
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok(); // Load .env file
    print_banner();

    let matches = App::new("Genesis")
        .version("1.0")
        .author("Your Name")
        .about("Sets up starter projects for Go or Rust")
        .arg(
            Arg::new("language")
                .short('l')
                .long("language")
                .value_name("LANGUAGE")
                .help("Sets the project language (go or rust)")
                .takes_value(true),
        )
        .get_matches();

    let language = match matches.value_of("language") {
        Some(lang) => lang.to_string(),
        None => get_language_selection()?,
    };

    match language.as_str() {
        "go" => setup_go_project()?,
        "rust" => setup_rust_project()?,
        _ => println!(
            "{}",
            style(format!("Unsupported language: {}", language)).red()
        ),
    }

    Ok(())
}

fn setup_go_project() -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", style("Setting up Go project...").yellow());

    println!("{}", style("Cloning repository...").cyan());
    let _repo = clone_repo(
        "https://github.com/ThembinkosiThemba/go-project-starter.git",
        "go-project",
    )?;

    println!("{}", style("Running setup commands...").cyan());
    Command::new("go")
        .arg("mod")
        .arg("tidy")
        .current_dir("go-project")
        .status()?;

    println!(
        "{}",
        style("Go project set up successfully!").green().bold()
    );
    Ok(())
}

fn setup_rust_project() -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", style("Setting up Rust project...").yellow());

    println!("{}", style("Cloning repository...").cyan());
    let _repo = clone_repo(
        "https://github.com/yourusername/private-rust-starter.git",
        "rust-project",
    )?;

    println!("{}", style("Running setup commands...").cyan());
    Command::new("cargo")
        .arg("build")
        .current_dir("rust-project")
        .status()?;

    println!(
        "{}",
        style("Rust project set up successfully!").green().bold()
    );
    Ok(())
}
