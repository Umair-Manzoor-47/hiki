use std::env;
use std::fmt::format;
use std::fs::{self, ReadDir};
use std::path::Path;
use colored::Colorize;

fn main() {
    let file_path = match get_path() {
        Ok(p) => p,
        Err(e) => {
            eprintln!("Error: {}", e);
            return;
        }
    };

    if let Err(e) = list_dir(&file_path) {
        eprintln!("Error: {}", e);
        return;
    }

    let problems = validate_unity_project(&file_path);
    if problems.is_empty() {
        println!("{}", "Valid Unity project.".green());
        if let Err(e) = generate_project_hierarchy(&file_path) {
            eprintln!("Error: {}", e);
            return;
        }

        println!("{}", "Project hierarchy generated.".green());
    } else {
        println!("{}", "Not a valid Unity project:".red());
        for p in &problems {
            println!("  - {}", p.yellow());
        }
    }
}

fn get_path() -> Result<std::path::PathBuf, Box<dyn std::error::Error>>{
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        return Err("No argument provided.".into());
    }
    if args[1].is_empty() {
        return Err("Path is empty.".into());
    }

    let path = Path::new(&args[1]);
    if !path.exists() {
        return Err("Path does not exist.".into());
    }

    Ok(path.to_path_buf())
}

fn list_dir(path: &Path) -> Result<(), Box<dyn std::error::Error>> {

    let entries: ReadDir = fs::read_dir(path)?;
    for entry in entries {
        match entry {
            Ok(e) => println!("Name: {}", e.path().display()),
            Err(e) => eprintln!("Error reading entry: {}", e),
        }
    }

    Ok(())
}

fn validate_unity_project(root: &Path) -> Vec<String> {
    let mut problems = Vec::new();
    if !root.is_dir() {
        return vec!["Given path is not a directory".to_string()];
    }
    if !root.join("Assets").is_dir() {
        problems.push("Missing Assets/ directory".to_string());
    }
    if !root.join("ProjectSettings").is_dir() {
        problems.push("Missing ProjectSettings/ directory".to_string());
    }
    if !root.join("Packages").is_dir() {
        problems.push("Missing Packages/ directory".to_string());
    }

    let project_version = root.join("ProjectSettings").join("ProjectVersion.txt");
    if !project_version.is_file(){
        problems.push("Missing ProjectSettings/ProjectVersion.txt".to_string());
    }
    let manifest = root.join("Packages").join("manifest.json");
    if !manifest.is_file() {
        problems.push("Missing Packages/manifest.json".to_string());
    } else {
        match fs::metadata(&manifest) {
            Ok(meta) if meta.len() == 0 => {
                problems.push("manifest.json is empty".to_string());
            }
            Err(e) => {
                problems.push(format!("Could not read manifest.json metadata: {}", e));
            }
            _ => {}
        }
    }

    problems
}

fn generate_project_hierarchy(path: &Path) -> Result<(), Box<dyn std::error::Error>>{

    let asset_path = path.join("Assets");
    let project_path = asset_path.join("_Project");
    if let Err(e) = fs::create_dir(&project_path) {
        return Err(format!("Failed to create _test directory: {}", e).into());
    }

    Ok(())
}

