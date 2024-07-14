use clap::{App, Arg};
use colored::*;
use console::{style, Term};
use dialoguer::{theme::ColorfulTheme, Input, Select};
use dirs::desktop_dir;
use dotenv::dotenv;
use git2::{build::RepoBuilder, Cred, FetchOptions, Progress, RemoteCallbacks, Repository};
use indicatif::{ProgressBar, ProgressStyle};
use std::{
    cell::RefCell,
    fs,
    io::{Error as IoError, Read, Write},
    path::Path,
    process::Command,
    rc::Rc,
};
use toml_edit::{Document, Item};
use walkdir::WalkDir;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok(); // Load .env file
    let term = Term::stdout();
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

    let path = matches
        .value_of("path")
        .unwrap_or(&default_path)
        .to_string();

    match language.as_str() {
        "go" => {
            let module_name = prompt_step(&term, "Enter your Go module name:", || {
                Ok(Input::<String>::new()
                    .with_prompt("(e.g., github.com/username/project)")
                    .interact_text()?)
            })?;
            setup_go_project(&path, &project_name, &module_name)?
        }
        "rust" => setup_rust_project(&path, &project_name)?,
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
    term.clear_last_lines(2)?; // Clear the previous question
    println!("{}", style(prompt).cyan().bold());
    let result = input_fn()?;
    term.clear_last_lines(1)?; // Clear the input line
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

fn clone_repo(url: &str, path: &str) -> Result<Repository, git2::Error> {
    let pb = Rc::new(RefCell::new(ProgressBar::new(100)));
    pb.borrow_mut().set_style(
        ProgressStyle::default_bar()
            .template(
                "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos:>7}/{len:7} {msg}",
            )
            .progress_chars("#>-"),
    );

    let pb_clone = pb.clone();

    let mut callbacks = RemoteCallbacks::new();
    callbacks.credentials(move |_url, _username_from_url, _allowed_types| {
        Cred::userpass_plaintext("git", &"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")
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

fn setup_go_project(
    base_path: &str,
    project_name: &str,
    module_name: &str,
) -> Result<(), Box<dyn std::error::Error>> {
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

    let old_module_name = "github.com/ThembinkosiThemba/go-project-starter";
    match update_module_name(&project_path, old_module_name, module_name) {
        Ok(_) => println!("Module name updated successfully."),
        Err(e) => println!(
            "Error updating module name: {}. Continuing with setup...",
            e
        ),
    }

    println!("{}", style("Running setup commands...").cyan());
    Command::new("go")
        .arg("mod")
        .arg("tidy")
        .current_dir(&project_path)
        .status()?;

    println!(
        "{}",
        style("Go project set up successfully!").green().bold()
    );
    Ok(())
}

fn update_module_name(
    project_path: &Path,
    old_module_name: &str,
    new_module_name: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    for entry in WalkDir::new(project_path)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();
        if path.is_file() {
            match update_file_content(path, old_module_name, new_module_name) {
                Ok(updated) => {
                    if updated {
                        println!("Updated module name in: {}", path.display());
                    }
                }
                Err(e) => println!("Error updating file {}: {}", path.display(), e),
            }
        }
    }
    Ok(())
}

fn update_file_content(
    path: &Path,
    old_module_name: &str,
    new_module_name: &str,
) -> Result<bool, IoError> {
    let mut content = Vec::new();
    let mut file = fs::File::open(path)?;
    file.read_to_end(&mut content)?;

    let old_bytes = old_module_name.as_bytes();
    let new_bytes = new_module_name.as_bytes();

    let mut updated = false;
    let mut new_content = Vec::new();

    let mut i = 0;
    while i < content.len() {
        if content[i..].starts_with(old_bytes) {
            new_content.extend_from_slice(new_bytes);
            i += old_bytes.len();
            updated = true;
        } else {
            new_content.push(content[i]);
            i += 1;
        }
    }

    if updated {
        let mut file = fs::File::create(path)?;
        file.write_all(&new_content)?;
    }

    Ok(updated)
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

    println!(
        "{}",
        style(format!(
            "NOTE: This is still a basic cargo project. More changes will be made soon.",
        ))
        .yellow()
    );

    let _repo = clone_repo(RUST_URL, project_path.to_str().unwrap())?;

    // Update Cargo.toml
    update_cargo_toml(&project_path, project_name)?;

    println!("{}", style("Running setup commands...").cyan());
    Command::new("cargo")
        .arg("build")
        .current_dir(&project_path)
        .status()?;

    println!(
        "{}",
        style("Rust project set up successfully!").green().bold()
    );
    Ok(())
}

fn update_cargo_toml(
    project_path: &Path,
    project_name: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let cargo_toml_path = project_path.join("Cargo.toml");
    let cargo_toml_content = fs::read_to_string(&cargo_toml_path)?;
    let mut doc = cargo_toml_content.parse::<Document>()?;

    if let Some(package) = doc.as_table_mut().get_mut("package") {
        if let Some(name) = package.get_mut("name") {
            *name = Item::Value(project_name.into());
        }
    }

    let updated_content = doc.to_string();
    let mut file = fs::File::create(&cargo_toml_path)?;
    file.write_all(updated_content.as_bytes())?;

    println!("Updated project name in Cargo.toml");
    Ok(())
}

const RUST_URL: &str = "https://github.com/ThembinkosiThemba/rust-project-starter.git";
const GO_URL: &str = "https://github.com/ThembinkosiThemba/go-project-starter.git";
