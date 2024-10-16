use std::path::PathBuf;
use std::thread;
use std::time::Duration;

mod constants;
mod file;
mod git;
mod setup;
mod utils;

use crate::setup::{setup_go_project, setup_rust_project, update_genesis};
use crate::utils::{print_banner, prompt_database_selection, prompt_step};

use clap::{Arg, Command};
use colored::*;
use console::{style, Term};
use dialoguer::{theme::ColorfulTheme, Input, Select};
use dirs::desktop_dir;
use dotenv::dotenv;
use indicatif::{ProgressBar, ProgressStyle};
use setup::setup_react_ts_vite_project;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let term = Term::stdout();
    print_banner();

    let default_path = desktop_dir()
        .expect("could not find desktop directory")
        .to_str()
        .expect("Desktop path is not valid UTF-8")
        .to_string();

    let spinner_style = ProgressStyle::default_spinner()
        .tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈ ")
        .template("{spinner:.green} {msg}");

    let progress_style = ProgressStyle::default_bar()
        .template(
            "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos:>7}/{len:7} {msg}",
        )
        .progress_chars("#>-");

    let matches = Command::new("Genesis")
        .version("1.2.0")
        .author("Thembinkosi Mkhonta")
        .about("Sets up starter projects for Go and Rust")
        .subcommand(Command::new("update").about("Updates genesis to the latest version"))
        .arg(
            Arg::new("language")
                .short('l')
                .long("language")
                .value_name("LANGUAGE")
                .help("Sets the project language (go, rust, or react)"),
        )
        // .arg(
        //     Arg::new("path")
        //         .short('p')
        //         .long("path")
        //         .value_name("PATH")
        //         .help("Sets the path where the project will be cloned")
        //         .default_value(&*default_path.clone()),
        // )
        .get_matches();

    if let Some(_) = matches.subcommand_matches("update") {
        return update_genesis();
    }

    let language = match matches.get_one::<String>("language").map(|s| s.as_str()) {
        Some(lang) => lang.to_string(),
        None => {
            let spinner = ProgressBar::new_spinner();
            spinner.set_style(spinner_style.clone());
            spinner.set_message("Preparing language options...");
            spinner.enable_steady_tick(100);
            thread::sleep(Duration::from_secs(1));
            spinner.finish_and_clear();

            prompt_step(&term, "Choose your project language:", || {
                let options = &["Go", "Rust", "React"];
                let selection = Select::with_theme(&ColorfulTheme::default())
                    .with_prompt("Select your preferred language")
                    .items(&options[..])
                    .default(0)
                    .interact_on(&term)?;
                Ok(options[selection].to_lowercase())
            })?
        }
    };

    let project_name = prompt_step(&term, "Enter your project name:", || {
        Ok(Input::<String>::new()
            .with_prompt("Project name")
            .interact_text()?)
    })?;

    let path = prompt_step(&term, "Enter the project path:", || {
        let input: String = Input::new()
            .with_prompt("Project path (press Enter for default)")
            .with_initial_text(&default_path)
            .allow_empty(true)
            .interact_text()?;

        if input.is_empty() {
            Ok(PathBuf::from(default_path))
        } else {
            Ok(PathBuf::from(shellexpand::tilde(&input).into_owned()))
        }
    })?;

    println!(
        "\n{}",
        "Project Configuration Summary:".bright_cyan().bold()
    );
    println!("  {} {}", "Language:".bright_yellow(), language);
    println!("  {} {}", "Project Name:".bright_yellow(), project_name);
    println!("  {} {}", "Path:".bright_yellow(), path.display());

    let confirm = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Do you want to proceed with this configuration?")
        .items(&["Yes, let's go!", "No, I want to start over"])
        .default(0)
        .interact_on(&term)?;

    if confirm == 1 {
        println!("{}", "Starting over...".bright_yellow());
        return Ok(());
    }

    let progress_bar = ProgressBar::new(100);
    progress_bar.set_style(progress_style);

    match language.as_str() {
        "go" => {
            let module_name = prompt_step(&term, "Enter your Go module name:", || {
                Ok(Input::<String>::new()
                    .with_prompt("Go module name (e.g., github.com/username/project)")
                    .interact_text()?)
            })?;
            let database = prompt_database_selection(&term)?;

            for i in 0..=100 {
                progress_bar.set_position(i);
                progress_bar.set_message(format!("Setting up Go project: {}%", i));
                thread::sleep(Duration::from_millis(50));
            }
            progress_bar.finish_with_message("Go project setup complete!");

            setup_go_project(
                path.to_str().unwrap(),
                &project_name,
                &module_name,
                &database,
            )?
        }
        "rust" => {
            for i in 0..=100 {
                progress_bar.set_position(i);
                progress_bar.set_message(format!("Setting up Rust project: {}%", i));
                thread::sleep(Duration::from_millis(50));
            }
            progress_bar.finish_with_message("Rust project setup complete!");

            setup_rust_project(path.to_str().unwrap(), &project_name)?
        }
        "react" => {
            for i in 0..=100 {
                progress_bar.set_position(i);
                progress_bar.set_message(format!("Setting up React project: {}%", i));
                thread::sleep(Duration::from_millis(50));
            }
            progress_bar.finish_with_message("React project setup complete!");

            setup_react_ts_vite_project(path.to_str().unwrap(), &project_name)?
        }
        _ => println!(
            "{}",
            style(format!("Unsupported language: {}", language)).red()
        ),
    }

    println!(
        "\n{}",
        "Project setup completed successfully!"
            .bright_green()
            .bold()
    );
    println!(
        "{}",
        format!(
            "Your new {} project '{}' is ready at {}",
            language,
            project_name,
            path.display()
        )
        .bright_cyan()
    );
    println!("\n{}", "Happy coding! 🚀".bright_yellow().bold());

    Ok(())
}
