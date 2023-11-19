pub mod cli;
pub mod counter;

use std::{
    io::{self, BufReader},
    fs::File,
};
use anyhow::Result;
use cli::Cli;
use counter::Counter;

pub fn run(cli: Cli, counter: &mut Counter) -> Result<()> {
    let flags = &cli.flags;
    let mut filename = String::new();

    // Reading file from env args, otherwise take in stdin
    if let Some(name) = &cli.file {
        filename = name.to_string();
        let file = File::open(name)?;
        let mut file_reader = BufReader::new(file);
        counter.load(&mut file_reader)?;
    } else {
        let stdin = io::stdin();
        let mut handle = stdin.lock();
        counter.load(&mut handle)?;
    }

    let mut out: Vec<usize> = Vec::new();

    if flags.chars {
        counter.count_chars()?;
        out.push(counter.chars());
    } else if flags.bytes {
        out.push(counter.bytes());
    }
    if flags.lines {
        out.push(counter.lines());
    }
    if flags.words {
        counter.count_words()?;
        out.push(counter.words());
    }

    if !flags.chars && !flags.bytes && !flags.words && !flags.lines {
        counter.default()?;
        out.push(counter.lines());
        out.push(counter.words());
        out.push(counter.bytes());
    }

    print!(" ");
    for val in out {
        print!("  {} ", val);
    }
    print!("{}\n", filename);

    Ok(())
}
