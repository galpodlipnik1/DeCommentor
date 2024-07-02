mod utils;
mod processor;

pub fn run (root: String) {
    println!("Printing all files in the directory: {}", root);
    let files = utils::walk_dir(root);
    for file in files {
        println!("{}",processor::get_file_contents(file).unwrap_or_default());
    }
}