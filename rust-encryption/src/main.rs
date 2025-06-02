use clap::{Parser};

#[derive(Parser)]
struct Args {
    #[clap(short, long)]
    file: String,
}

fn main() {
    let args = Args::parse();
    println!("Processing file: {}", args.file);
    // Call encryption and packaging functions here
}
