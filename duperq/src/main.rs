use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Query to run.
    query: String,

    /// Files to read from. If missing, defaults to stdin.
    blob: Option<String>,
}

fn main() {
    println!("Hello, world!");
}
