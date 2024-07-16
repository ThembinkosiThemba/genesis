use std::{
    fs,
    io::{Error as IoError, Read, Write},
    path::Path,
};
use walkdir::WalkDir;

pub fn update_module_name(
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
