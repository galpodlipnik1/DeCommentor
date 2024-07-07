use std::{fs, io::{Error, ErrorKind}, path::Path};
use colored::*;
use walkdir::WalkDir;
use crate::processor::QuoteStyle;

#[allow(dead_code)]
pub struct File {
    pub name: String,
    pub path: String,
    pub size: u64,
    pub extension: String,
    pub is_modified: bool,
}

impl File {
    pub fn new(name: String, path: String, size: u64, extension: String, is_modified: bool) -> File {
        File {
            name,
            path,
            size,
            extension,
            is_modified,
        }
    }
}

impl Clone for File {
    fn clone(&self) -> Self {
        Self { name: self.name.clone(), path: self.path.clone(), size: self.size.clone(), extension: self.extension.clone(), is_modified: self.is_modified.clone() }
    }
}

pub fn find_config_path() -> Result<String, String> {
    let mut config_path = String::new();
    for entry in WalkDir::new(".").into_iter().filter(|e| e.is_ok()) {
        let entry = entry.unwrap();
        if entry.file_name().to_string_lossy().contains(".neatify.json") {
            config_path = entry.path().to_str().unwrap().to_string();
            break;
        }
    }

    if config_path.is_empty() {
        return Err("No config file found in the current directory".to_string());
    }

    Ok(config_path)
} 

pub fn is_dir(file: &File) -> bool {
    fs::metadata(&file.path).unwrap().is_dir()
}

pub fn walk_dir(root: &String, ignored_files: Vec<String>) -> Result<Vec<File>, String> {
    let mut files: Vec<File> = Vec::new();

    for file in WalkDir::new(root).into_iter().filter_map(|e| e.ok()) {
        let name = file.file_name().to_string_lossy().to_string();

        if ignored_files.contains(&name) {
            continue;
        }

        let path = file.path().display().to_string().replace("\\", "/");
        let size = file.metadata().unwrap().len();
        let extension = file.path().extension().unwrap_or_default().to_string_lossy().to_string();
        let is_modified = false;

        let file = File::new(name, path, size, extension, is_modified);
        if !is_dir(&file) {
            files.push(file);
        }
    }

    if files.is_empty() {
        return Err("No files found in the directory".to_string());
    }

    Ok(files)
}

#[allow(dead_code)]
pub fn move_example_files() {
    println!("{}", "Moving example files...".bold().red());

    let examples_dir = "./examples";

    if let Ok(entries) = fs::read_dir(examples_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() {
                match fs::remove_file(&path) {
                    Ok(_) => println!("Removed file: {}", path.display()),
                    Err(e) => println!("Error removing file {}: {}", path.display(), e),
                }
            }
        }
    }

    let example_files = WalkDir::new("./example_files").into_iter().filter_map(|e| e.ok());
    for file in example_files {
        let path = file.path().display().to_string().replace("\\", "/");
        let new_path = path.replace("example_files", "examples");
        match fs::copy(&path, &new_path) {
            Ok(_) => println!("Copied file to: {}", new_path),
            Err(e) => println!("Error copying file to {}: {}", new_path, e),
        }
    }

    println!("{}", "Example files moved successfully".bold().green());
}

pub fn str_to_quote_style(s: String) -> Option<QuoteStyle> {
    match s.as_str() {
        "single" => Some(QuoteStyle::Single),
        "double" => Some(QuoteStyle::Double),
        _ => None,
    }
}

pub fn write_to_file(path: &str, content: &str) -> Result<bool, std::io::Error> {
    let path = Path::new(path);
    if !path.exists() {
        return Err(Error::new(ErrorKind::NotFound, "File not found"));
    }

    fs::write(path, content).map_err(|e| Error::new(ErrorKind::Other, e))?;

    Ok(true)
}