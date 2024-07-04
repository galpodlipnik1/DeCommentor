use colored::*;

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
        println!("{}{}", "Processing:".bold(), format!(" File: {}, Size: {} bytes", file.name, file.size).blue());
        let content = processor::get_file_contents(&file).unwrap_or_default();
        let new_file = processor::remove_comments(content.clone(), file).unwrap_or_else(|e| {
            println!("Error processing file: {:?}", e);
            std::process::exit(1);
        });
        println!("{}{}", "Result".bold(), format!(" File: {}, Size: {} bytes, Was modified: {}", new_file.name, new_file.size, new_file.is_modified).green());
    }
}