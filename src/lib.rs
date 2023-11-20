pub mod cli;
pub mod counter;

use std::{
    io::{self, BufReader},
    fs::File,
};
use anyhow::Result;
use cli::Cli;
use counter::Counter;

const BUF_SIZE: usize = 1024 * 16;

pub fn run(cli: Cli, counter: &mut Counter) -> Result<()> {
    let mut filename = String::new();

    // Reading file from env args, otherwise take in stdin
    if let Some(name) = &cli.file {
        filename = name.to_string();
        let file = File::open(name)?;
        let mut file_reader = BufReader::with_capacity(BUF_SIZE, file);
        counter.load(&mut file_reader)?;
    } else {
        let stdin = io::stdin();
        let mut handle = stdin.lock();
        counter.load(&mut handle)?;
    }

    print!(" ");

    if cli.chars {
        print!("  {} ", counter.chars());
    } else if cli.bytes {
        print!("  {} ", counter.bytes());
    }
    if cli.lines {
        print!("  {} ", counter.lines());
    }
    if cli.words {
        print!("  {} ", counter.words());
    }

    if !cli.chars && !cli.bytes && !cli.words && !cli.lines {
        print!("  {} ", counter.lines());
        print!("  {} ", counter.words());
        print!("  {} ", counter.bytes());
    }

    print!("{}\n", filename);

    Ok(())
}
