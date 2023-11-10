use clap::{Parser, Args};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Flags for file parsing
    #[command(flatten)]
    pub flags: Flag,

    /// The file to analyze
    pub file: Option<String>,
}

#[derive(Args)]
#[group(required = true, multiple = false)]
pub struct Flag {
    /// Print the number of bytes in the file
    #[arg(short = 'c', long)]
    bytes: bool,

    /// Print the number of lines in the file
    #[arg(short, long)]
    lines: bool,

    /// Print the number of words in the file
    #[arg(short, long)]
    words: bool,

    /// Print the number of chars in the file
    #[arg(short = 'm', long)]
    chars: bool,
}

