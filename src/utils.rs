use console::{style, Term};
use dialoguer::{theme::ColorfulTheme, Select};

use colored::*;
use regex;
use std::{fs, io::Write, path::Path};
use toml_edit::{Document, Item};

pub fn prompt_step<T>(
    term: &Term,
    prompt: &str,
    input_fn: impl FnOnce() -> Result<T, Box<dyn std::error::Error>>,
) -> Result<T, Box<dyn std::error::Error>> {
    term.clear_last_lines(2)?;
    println!("{}", style(prompt).cyan().bold());
    let result = input_fn()?;
    term.clear_last_lines(1)?;
    println!("{} {}", style("✓").green().bold(), style(prompt).dim());
    Ok(result)
}

pub fn update_cargo_toml(
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

    println!("{}", style("Updated project name in Cargo.toml!").green());
    Ok(())
}

pub fn prompt_database_selection(term: &Term) -> Result<String, Box<dyn std::error::Error>> {
    prompt_step(term, "Choose your database:", || {
        let options = &["MongoDB", "PostgreSQL"];
        let selection = Select::with_theme(&ColorfulTheme::default())
            .items(&options[..])
            .default(0)
            .interact_on(term)?;
        Ok(options[selection].to_lowercase())
    })
}

pub fn print_banner() {
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

pub fn update_database_config(
    project_path: &Path,
    database: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let main_go_path = project_path.join("cmd").join("main.go");
    if main_go_path.exists() {
        let mut content = fs::read_to_string(&main_go_path)?;

        let new_init_code = if database == "mongodb" {
            r#"userRepo, err := config.InitializeRepositoriesMongo()
    if err != nil {
        log.Fatal(err)
    }

    // Initialize user usecase with MongoDB repository
    userUsecase := config.InitializeUsecasesMongo(userRepo)"#
        } else {
            r#"userRepo, err := config.InitializeRepositoriesPostgres()
    if err != nil {
        log.Fatal(err)
    }

    // Initialize user usecase with PostgreSQL repository
    userUsecase := config.InitializeUsecasesPostgres(userRepo)"#
        };

        let old_mongo_code = r#"userRepo, err := config.InitializeRepositoriesMongo()
    if err != nil {
        log.Fatal(err)
    }

    // Initialize user usecase with MongoDB repository
    userUsecase := config.InitializeUsecasesMongo(userRepo)"#;

        let old_postgres_code = r#"userRepo, err := config.InitializeRepositoriesPostgres()
    if err != nil {
        log.Fatal(err)
    }

    // Initialize user usecase with PostgreSQL repository
    userUsecase := config.InitializeUsecasesPostgres(userRepo)"#;

        if content.contains(old_mongo_code) || content.contains(old_postgres_code) {
            content = content.replace(old_mongo_code, new_init_code);
            content = content.replace(old_postgres_code, new_init_code);
            fs::write(main_go_path, content)?;
            println!(
                "{}",
                style("Updated main.go with selected database").green()
            );
        } else {
            println!("{}", style("Couldn't find expected database initialization in main.go. Manual update may be required.").yellow());
        }
    } else {
        println!(
            "{}",
            style("main.go not found in cmd directory. Skipping update.").yellow()
        );
    }

    // Update user.go
    let user_go_path = project_path
        .join("internal")
        .join("application")
        .join("usecases")
        .join("user")
        .join("user.go");
    if user_go_path.exists() {
        let mut content = fs::read_to_string(&user_go_path)?;

        let new_user_code = if database == "mongodb" {
            r#"type UserUsecase struct {
	userRepo mongodb.Interface
}

// NewUserUsecase creates a new UserUsecase instance.
// It takes a mongodb.Interface as a parameter to handle database operations.
func NewUserUsecase(repo mongodb.Interface) *UserUsecase {
	return &UserUsecase{userRepo: repo}
}"#
        } else {
            r#"type UserUsecase struct {
	userRepo postgres.Interface
}

// NewUserUsecase creates a new UserUsecase instance.
// It takes a postgres.Interface as a parameter to handle database operations.
func NewUserUsecase(repo postgres.Interface) *UserUsecase {
	return &UserUsecase{userRepo: repo}
}"#
        };

        // let old_user_code_regex = regex::Regex::new(r"type UserUsecase struct \{[\s\S]*?func NewUserUsecase\([^)]*\) \*UserUsecase \{[\s\S]*?\}").unwrap();
        let old_user_code_regex = regex::Regex::new(r"(?m)^type UserUsecase struct \{[\s\S]*?^func NewUserUsecase\([^)]*\) \*UserUsecase \{[\s\S]*?^\}").unwrap();
        if old_user_code_regex.is_match(&content) {
            content = old_user_code_regex
                .replace_all(&content, new_user_code)
                .to_string();
            fs::write(user_go_path, content)?;
            println!(
                "{}",
                style("Updated user.go with selected database").green()
            );
        } else {
            println!("{}", style("Couldn't find expected UserUsecase struct in user.go. Manual update may be required.").yellow());
        }
    } else {
        println!("{}", style("user.go not found. Skipping update.").yellow());
    }

    Ok(())
}
