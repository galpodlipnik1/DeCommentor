use clap::Parser;
use std::time::Instant;
use neatify::run;
use utils::{find_config_path, move_example_files};

mod processor;
mod constants;
mod utils;
mod tests;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    config: Option<String>,

    #[arg(short, long)]
    dev: bool,
}

fn main() {
    let start = Instant::now();

    let args = Args::parse();
    
    let config_path;

    if args.dev {
        println!("Running in dev mode");
        move_example_files();
        config_path = r".\.neatify.json".to_string();
    } else {
        config_path = match find_config_path() {
            Ok(path) => path,
            Err(e) => {
                println!("{}", e);
                std::process::exit(1);
            }
        };
    }
    
    println!("Using config file: {}", config_path);

    run(config_path);

    let duration = start.elapsed();
    println!("Time elapsed: {:?}", duration);
}