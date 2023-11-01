use crate::cli::Flags::{BYTES, LINES, WORDS, CHARS, ALL};
use clap::Parser;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Print the number of bytes in the file
    #[arg(short = 'c', long)]
    pub bytes: bool,

    /// Print the number of lines in the file
    #[arg(short, long)]
    pub lines: bool,

    /// Print the number of words in the file
    #[arg(short, long)]
    pub words: bool,

    /// Print the number of chars in the file
    #[arg(short = 'm', long)]
    pub chars: bool,

    /// The file to analyze
    pub file: Option<String>,
}

pub enum Flags {
    BYTES,
    LINES,
    WORDS,
    CHARS,
    ALL,
}

impl Cli {
    // There is certainly a cleaner way to do all this...
    pub fn get_flags(&self) -> Vec<Flags> {
        let mut flags: Vec<Flags> = Vec::new();

        if self.bytes && !self.chars {
            flags.push(BYTES);
        }
        if self.lines {
            flags.push(LINES);
        }
        if self.words {
            flags.push(WORDS);
        }
        if self.chars {
            flags.push(CHARS);
        }

        if !self.bytes && !self.lines && !self.words && !self.chars {
            flags.push(ALL);
        }

        flags
    }
}

