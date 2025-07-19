// src/main.rs
/*
 * Main executable for AdtechplatformsEngine
 */

use clap::Parser;
use adtechplatformsengine::{Result, run};

#[derive(Parser)]
#[command(version, about = AdtechplatformsEngine - A Rust implementation)]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose)
}
