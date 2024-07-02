mod utils;

pub fn run (root: String) {
    println!("Printing all files in the directory: {}", root);
    let files = utils::walk_dir(root);
    println!("Files: {:#?}", files)
}