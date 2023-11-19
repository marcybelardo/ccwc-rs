use ccwc_rs::{cli::Cli, counter::Counter, run};

use anyhow::Result;
use clap::Parser;

fn main() -> Result<()> {
    let args = Cli::parse();
    let mut counter = Counter::new();

    if let Err(e) = run(args, &mut counter) {
        eprintln!("Error: {e}");
    }

    Ok(())
}
