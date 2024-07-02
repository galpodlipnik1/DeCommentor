use crate::utils::File;
use std::{fs::File as FsFile, io::Read, io::ErrorKind, fs};

fn is_dir(file: &File) -> bool {
    fs::metadata(&file.path).unwrap().is_dir()
}


pub fn get_file_contents(file: File) -> Option<String> {
    if is_dir(&file) {
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