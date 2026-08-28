use std::env;
use std::fs::{self, ReadDir};

fn main() {
    let paths = read_path();

    if paths.is_err() {
        println!("Error: {}", paths.unwrap_err());
        return;
    }

    let paths = paths.unwrap();

    for path in paths {
        println!("Name: {}", path.unwrap().path().display());
    }
}

fn read_path() -> Result<ReadDir, String> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        return Err("No argument provided.".to_string());
    }

    let file_path = &args[1];

    if file_path.is_empty() {
        return Err("Path is empty.".to_string());
    }

    if !std::path::Path::new(file_path).exists() {
        return Err("Path does not exist.".to_string());
    }

    let paths = fs::read_dir(file_path);

    if paths.is_err() {
        return Err("Could not read directory.".to_string());
    }

    Ok(paths.unwrap())
}
