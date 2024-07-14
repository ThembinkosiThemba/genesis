use clap::{App, Arg};
use colored::*;
use console::{style, Term};
use dialoguer::{theme::ColorfulTheme, Select};
use dirs::desktop_dir;
use dotenv::dotenv;
use git2::{build::RepoBuilder, Cred, FetchOptions, Progress, RemoteCallbacks, Repository};
use indicatif::{ProgressBar, ProgressStyle};
use std::cell::RefCell;
use std::path::Path;
use std::rc::Rc;

use std::process::Command;

const RUST_URL: &str = "https://github.com/ThembinkosiThemba/rust-project-starter.git";
const GO_URL: &str = "https://github.com/ThembinkosiThemba/go-project-starter.git";

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

    let default_path = desktop_dir()
        .expect("could not find desktop directory")
        .to_str()
        .expect("Desktop path is not valid UTF-8")
        .to_string();

    let matches = App::new("Genesis")
        .version("1.0.0")
        .author("Thembinkosi Mkhonta")
        .about("Sets up starter projects for Go or Rust")
        .arg(
            Arg::new("language")
                .short('l')
                .long("language")
                .value_name("LANGUAGE")
                .help("Sets the project language (go or rust)")
                .takes_value(true),
        )
        .arg(
            Arg::new("path")
                .short('p')
                .long("path")
                .value_name("PATH")
                .help("Sets the path where the project will be cloned")
                .default_value(&default_path)
                .takes_value(true),
        )
        .get_matches();

    let language = match matches.value_of("language") {
        Some(lang) => lang.to_string(),
        None => get_language_selection()?,
    };

    let path = matches.value_of("path").unwrap().to_string();
    let project_name = get_project_name(&language)?;

    match language.as_str() {
        "go" => setup_go_project(&path, &project_name)?,
        "rust" => setup_rust_project(&path, &project_name)?,
        _ => println!(
            "{}",
            style(format!("Unsupported language: {}", language)).red()
        ),
    }

    Ok(())
}

fn get_project_name(language: &str) -> Result<String, Box<dyn std::error::Error>> {
    let project_name = dialoguer::Input::<String>::new()
        .with_prompt(format!("Enter your {} project name", language))
        .interact_text()?;
    Ok(project_name)
}

fn setup_go_project(base_path: &str, project_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", style("Setting up Go project...").yellow());

    let project_path = Path::new(base_path).join(project_name);
    println!(
        "{}",
        style(format!(
            "Cloning repository to {}...",
            project_path.display()
        ))
        .cyan()
    );

    let _repo = clone_repo(GO_URL, project_path.to_str().unwrap())?;

    println!("{}", style("Running setup commands...").cyan());
    Command::new("go")
        .arg("mod")
        .arg("tidy")
        .current_dir(project_path)
        .status()?;

    println!(
        "{}",
        style("Go project set up successfully!").green().bold()
    );
    Ok(())
}

fn setup_rust_project(
    base_path: &str,
    project_name: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", style("Setting up Rust project...").yellow());

    let project_path = Path::new(base_path).join(project_name);
    println!(
        "{}",
        style(format!(
            "Cloning repository to {}...",
            project_path.display()
        ))
        .cyan()
    );

    let _repo = clone_repo(RUST_URL, project_path.to_str().unwrap())?;

    println!("{}", style("Running setup commands...").cyan());
    Command::new("cargo")
        .arg("build")
        .current_dir(project_path)
        .status()?;

    println!(
        "{}",
        style("Rust project set up successfully!").green().bold()
    );
    Ok(())
}
