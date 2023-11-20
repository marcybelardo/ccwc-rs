use ccwc_rs::{cli::Cli, counter::Counter, BUF_SIZE};

use anyhow::Result;
use clap::Parser;
use std::{
    fs::File,
    io::{self, BufReader},
};

fn main() -> Result<()> {
    let cli = Cli::parse();
    let mut counter = Counter::new();

    let mut filename = String::new();

    // Reading file from env args if there is, otherwise take in stdin
    if let Some(name) = &cli.file {
        filename = name.to_string();
        let file = File::open(name)?;
        let mut file_reader = BufReader::with_capacity(BUF_SIZE, file);
        counter.count_complicated(&mut file_reader)?;
    } else {
        let stdin = io::stdin();
        let mut handle = stdin.lock();
        counter.count_complicated(&mut handle)?;
    }

    counter.print_results(&filename, cli.chars, cli.bytes, cli.lines, cli.words);

    Ok(())
}
