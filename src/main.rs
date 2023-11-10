use std::io;

use ccwc_rs::{
    cli::Cli,
    counter::Counter,
};

use anyhow::Result;
use clap::Parser;

fn main() -> Result<()> {
    let args = Cli::parse();
    let flags = &args.flags;
    let mut counter = Counter::new();
    let mut filename = String::new();

    // Reading file from env args, otherwise take in stdin
    if let Some(file) = &args.file {
        filename = file.to_string();
        counter.load_from_file(file)?;
    } else {
        loop {
            match io::stdin().read_line(&mut counter.contents) {
                Ok(len) => {
                    if len == 0 {
                        break;
                    } else {
                        continue;
                    }
                }
                Err(e) => {
                    eprintln!("error: {}", e);
                    break;
                }
            }
        }
    }

    let mut out: Vec<usize> = Vec::new();

    match (flags.bytes, flags.lines, flags.words, flags.chars) {
        (true, false, false, false | true) => counter.bytes(),
        (false, true, false, false) => counter.lines(),
        (false, false, true, false) => counter.words(),
    }

    print!(" ");
    for val in out {
        print!("  {} ", val);
    }
    print!("{}\n", filename);

    Ok(())
}
