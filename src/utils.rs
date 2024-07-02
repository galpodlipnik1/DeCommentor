use walkdir::WalkDir;

pub struct File {
    name: String,
    path: String,
    size: u64,
    extension: String,
    is_modified: bool,
}

impl File {
    fn new(name: String, path: String, size: u64, extension: String, is_modified: bool) -> File {
        File {
            name,
            path,
            size,
            extension,
            is_modified,
        }
    }
}

pub fn walk_dir(root: String) -> Vec<File> {
    let mut files: Vec<File> = Vec::new();

    for file in WalkDir::new(root).into_iter().filter_map(|e| e.ok()) {
        let name = file.file_name().to_string_lossy().to_string();
        let path = file.path().display().to_string();
        let size = file.metadata().unwrap().len();
        let extension = file.path().extension().unwrap().to_string_lossy().to_string();
        let is_modified = false;

        let file = File::new(name, path, size, extension, is_modified);
        files.push(file);
    }

    files
}