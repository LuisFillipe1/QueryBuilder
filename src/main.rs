// src/main.rs
/*
 * Main executable for QueryBuilder
 */

use clap::Parser;
use querybuilder::{Result, run};

#[derive(Parser)]
#[command(version, about = "QueryBuilder - A Rust implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
    
    /// Input file path
    #[arg(short, long)]
    input: Option<String>,
    
    /// Output file path
    #[arg(short, long)]
    output: Option<String>,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose, args.input, args.output)
}
