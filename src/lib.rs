mod utils;
mod processor;
mod constants;

pub fn run(root: String) {
    let files = utils::walk_dir(&root);

    if files.is_empty() {
        println!("No files found in the directory: {}", root);
        return;
    }
    
    println!("Printing all files in the directory: {}", root);
    for file in files {
        println!("Checking file: {}", file.name);
        let content = processor::get_file_contents(&file).unwrap_or_default();
        let (_new_content, new_file) = processor::remove_comments(content.clone(), file);
        println!("File: {}, Is modified: {}", new_file.name, new_file.is_modified);
    }
}