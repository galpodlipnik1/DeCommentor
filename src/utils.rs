use std::fs;

use walkdir::WalkDir;

#[derive(Debug)]
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

pub fn is_dir(file: &File) -> bool {
    fs::metadata(&file.path).unwrap().is_dir()
}

pub fn walk_dir(root: &String) -> Vec<File> {
    let mut files: Vec<File> = Vec::new();

    for file in WalkDir::new(root).into_iter().filter_map(|e| e.ok()) {
        let name = file.file_name().to_string_lossy().to_string();
        let path = file.path().display().to_string().replace("\\", "/");
        let size = file.metadata().unwrap().len();
        let extension = file.path().extension().unwrap_or_default().to_string_lossy().to_string();
        let is_modified = false;

        let file = File::new(name, path, size, extension, is_modified);
        if !is_dir(&file) {
            files.push(file);
        }
    }

    files
}

#[allow(dead_code)]
pub fn remove_example_files() { 
    println!("Removing all pretty files from examples directory");
    let files = walk_dir(&String::from("examples"));
    println!();
    for file in files {
        if file.name.contains("pretty") {
            println!("{:?}", file);
            match std::fs::remove_file(&file.path) {
                Ok(_) => println!("Successfully removed file: {:?}", file.path),
                Err(e) => println!("Failed to remove file: {:?}, error: {:?}", file.path, e),
            }
        }
    }
}

#[allow(dead_code)]
pub fn beautify(content: String) -> String {
    content.lines().filter(|line| !line.trim().is_empty()).collect::<Vec<&str>>().join("\n")
}