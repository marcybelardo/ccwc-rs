use std::io;

use ccwc_rs::{
    cli::{Cli, Flags},
    counter::Counter,
};

use anyhow::Result;
use clap::Parser;

fn main() -> Result<()> {
    let args = Cli::parse();
    let flags = args.get_flags();
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

    for flag in flags {
        match flag {
            Flags::WORDS => {
                counter.count_words()?;
                out.push(counter.words());
            }
            Flags::LINES => {
                counter.count_lines()?;
                out.push(counter.lines());
            }
            Flags::BYTES => {
                counter.count_bytes()?;
                out.push(counter.bytes());
            }
            Flags::CHARS => {
                counter.count_chars()?;
                out.push(counter.chars());
            }
            Flags::ALL => {
                counter.count_bytes()?;
                counter.count_lines()?;
                counter.count_words()?;
                out.push(counter.lines());
                out.push(counter.words());
                out.push(counter.bytes());
            }
        }
    }

    print!(" ");
    for val in out {
        print!("  {} ", val);
    }
    print!("{}\n", filename);

    Ok(())
}
