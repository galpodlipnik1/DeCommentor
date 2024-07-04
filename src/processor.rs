use crate::utils::{beautify, File};
use std::{fs::{self, File as FsFile}, io::{ErrorKind, Read, Write}, path::Path};
use crate::constants::COMMENT_REGEX;

fn is_dir(file: &File) -> bool {
    fs::metadata(&file.path).unwrap().is_dir()
}


pub fn get_file_contents(file: &File) -> Option<String> {    
    if is_dir(file) {
        return None;
    }
    
    let mut file = match FsFile::open(&file.path) {
        Ok(file) => file,
        Err(e) => {
            if e.kind() == ErrorKind::PermissionDenied {
                println!("Permission denied when opening file: {}", file.path);
            } else {
                println!("Error opening file: {}", e);
            }
            return None;
        },
    };
    let mut contents = String::new();
    match file.read_to_string(&mut contents) {
        Ok(_) => Some(contents),
        Err(e) => {
            println!("Error reading file: {}", e);
            None
        },
    }
}

pub fn remove_comments(content: String, file: File) -> (String, File) {
    let mut new_content = String::new();
    let lines = content.split('\n');
    for line in lines {
        if !COMMENT_REGEX.is_match(line.trim()) {
            new_content.push_str(line);
            new_content.push('\n'); 
        }
    }
    let beautified_content = beautify(new_content);

    // Ensure the file extension is correctly handled
    let extension = if file.extension.is_empty() { "" } else { &file.extension };
    let new_file_name = format!("{}_pretty{}", file.name.split('.').next().unwrap_or_default(), extension);

    // Correctly handle the directory path
    let dir_path = Path::new(&file.path).parent().unwrap_or_else(|| Path::new(""));
    let new_file_path = dir_path.join(&new_file_name);

    let mut new_file = FsFile::create(&new_file_path).expect("Unable to create file");
    new_file.write_all(beautified_content.as_bytes()).expect("Unable to write data");

    // Update the File struct with the new path and potentially other attributes
    let updated_file = File::new(file.name, new_file_path.to_str().unwrap().to_string(), file.size, file.extension, true);

    (beautified_content, updated_file)
}