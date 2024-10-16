use console::style;
use dialoguer::theme::ColorfulTheme;
use dialoguer::Select;
use std::io;
use std::{path::Path, process::Command};

use crate::constants::{GO_URL, OLD_MODULE_NAME, REACT_VITE_TYPESCRIPT_URL, RUST_URL};
use crate::file::update_module_name;
use crate::git::clone_repo;
use crate::utils::update_cargo_toml;
use crate::utils::update_database_config;

pub fn setup_react_ts_vite_project(
    base_path: &str,
    project_name: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    println!(
        "{}",
        style("Setting up react + Vite + Typescript project...").yellow()
    );
    println!();

    let project_path = Path::new(base_path).join(project_name);
    println!(
        "{}",
        style(format!(
            "Cloning repository to {}...",
            project_path.display()
        ))
        .cyan()
    );
    println!();

    let _repo = clone_repo(REACT_VITE_TYPESCRIPT_URL, project_path.to_str().unwrap())?;

    Command::new("npm")
        .arg("install")
        .arg("--legacy-peer-deps")
        .current_dir(&project_path)
        .status()?;

    println!();

    println!(
        "{}",
        style("React project set up successfully!").green().bold()
    );
    Ok(())
}

pub fn setup_go_project(
    base_path: &str,
    project_name: &str,
    module_name: &str,
    database: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", style("Setting up Go project...").yellow());
    println!();

    let project_path = Path::new(base_path).join(project_name);
    println!(
        "{}",
        style(format!(
            "Cloning repository to {}...",
            project_path.display()
        ))
        .cyan()
    );
    println!();

    let _repo = clone_repo(GO_URL, project_path.to_str().unwrap())?;

    let old_module_name = OLD_MODULE_NAME;
    match update_module_name(&project_path, old_module_name, module_name) {
        Ok(_) => println!("Module name updated successfully."),
        Err(e) => println!(
            "Error updating module name: {}. Continuing with setup...",
            e
        ),
    }

    // Remove unused database folder
    // remove_unused_database_folder(&project_path, database)?;

    // Update main.go with the selected database
    // update_main_go(&project_path, database)?;
    update_database_config(&project_path, database)?;
    println!("{}", style("Running setup commands...").cyan());

    Command::new("go")
        .arg("mod")
        .arg("tidy")
        .current_dir(&project_path)
        .status()?;
    println!();

    // let _ = remove_dot_git_dir(&project_path);

    println!(
        "{}",
        style("Go project set up successfully!").green().bold()
    );
    Ok(())
}

pub fn setup_rust_project(
    base_path: &str,
    project_name: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", style("Setting up Rust project...").yellow());
    println!();

    let project_path = Path::new(base_path).join(project_name);

    let options = &["Basic Rust Project", "Full Starter template"];
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Choose your Rust project type:")
        .items(&options[..])
        .default(0)
        .interact()?;

    match selection {
        0 => setup_basic_rust_project(&project_path, project_name),
        1 => setup_full_rust_project(&project_path, project_name),
        _ => unreachable!(),
    }
}

fn setup_basic_rust_project(
    project_path: &Path,
    project_name: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", style("Creating a basic Rust project...").cyan());

    Command::new("cargo")
        .arg("new")
        .arg(project_path)
        .status()?;

    // let _ = remove_dot_git_dir(project_path);

    println!(
        "{}",
        style(format!(
            "Basic Rust project '{}' created successfully!",
            project_name
        ))
        .green()
        .bold()
    );
    Ok(())
}

fn setup_full_rust_project(
    project_path: &Path,
    project_name: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", style("Setting up Rust project...").yellow());
    println!();

    // let project_path = Path::new(base_path).join(project_name);
    println!(
        "{}",
        style(format!(
            "Cloning repository to {}...",
            project_path.display()
        ))
        .cyan()
    );

    println!();

    println!(
        "{}",
        style(format!(
            "NOTE: Some changes have been made to the project. Check README for more info, or simple run `cargo build` and run.",
        ))
        .yellow()
    );

    println!();

    let _repo = clone_repo(RUST_URL, project_path.to_str().unwrap())?;

    // Update Cargo.toml
    update_cargo_toml(&project_path, project_name)?;

    println!("{}", style("Running setup commands...").cyan());
    Command::new("cargo")
        .arg("build")
        .current_dir(&project_path)
        .status()?;

    // let _ = remove_dot_git_dir(project_path);

    println!(
        "{}",
        style("Rust project set up successfully!").green().bold()
    );
    Ok(())
}

pub fn _remove_dot_git_dir(project_path: &Path) -> io::Result<()> {
    Command::new("rm")
        .arg("-rf")
        .arg(".git")
        .current_dir(&project_path)
        .status()?;
    Ok(())
}

pub fn update_genesis() -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", style("Checking for updates...").yellow());

    let output = Command::new("cargo")
        .args(&["install", "genesis_rs"])
        .output()?;

    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        if stdout.contains("Replacing") {
            println!(
                "{}",
                style("Update successful! Genesis has been updated to the latest version.").green()
            );
        } else {
            println!("{}", style("Genesis is already up to date.").yellow());
        }
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        println!("Update failed. Error: {}", stderr);
    }

    Ok(())
}
