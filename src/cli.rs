use clap::Parser;

#[derive(Parser)]
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

