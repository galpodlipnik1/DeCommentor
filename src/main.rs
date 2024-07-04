use clap::Parser;
use std::time::Instant;
use de_commentor::run;
use utils::remove_example_files;


mod utils;
mod tests;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    path: String,
}

fn main() {
    let start = Instant::now();

    let args = Args::parse();
    
    //REMOVE BEFORE PROD:
    remove_example_files();
    
    run(args.path);

    let duration = start.elapsed();
    println!("Time elapsed: {:?}", duration);
}