mod utils;

pub fn run (root: String) {
    println!("Printing all files in the directory: {}", root);
    utils::walk_dir(root);
}