use clap::Parser;
use std::time::Instant;
use de_commentor::run;

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

    run(args.path);

    let duration = start.elapsed();
    println!("Time elapsed: {:?}", duration);
}