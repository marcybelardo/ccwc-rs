use crate::BUF_SIZE;

use std::io::BufRead;
use anyhow::Result;

pub struct Counter {
    bytes: usize,
    lines: usize,
    words: usize,
    chars: usize,
}

impl Counter {
    pub fn new() -> Self {
        Self {
            bytes: 0,
            lines: 0,
            words: 0,
            chars: 0,
        }
    }

    // Counts everything
    pub fn count_complicated<R: BufRead>(&mut self, reader: &mut R) -> Result<()> {
        // Set word counter flag outside loop to take care of words that
        // are cut by the buffer size
        let mut is_word = false;

        loop {
            // It can probably infer the type from the initializer
            // Better safe than sorry lmao
            let mut contents: [u8; BUF_SIZE] = [0u8; BUF_SIZE];

            match reader.read(&mut contents) {
                Ok(len) if len == 0 => break,
                Ok(len) => {
                    // Count bytes by adding length of read bytes
                    self.bytes += len;

                    // I could use a for loop here instead
                    // But I don't want to
                    contents.iter().for_each(|byte| {
                        // We're not really using chars right now
                        // Besides, I'd have to do some extra saucy stuff
                        // for UTF-8 chars bigger than a byte
                        // self.chars += 1;
                        
                        // Count lines by looking for newline chars
                        if *byte == b'\n' { self.lines += 1 };

                        // Count words by counting word starts, i.e.
                        // `is_word` is first set to false, and is only
                        // flipped to true if a whitespace char is found.
                        // Then, if another whitespace char is found, we can
                        // say a word has been traversed, and the count
                        // is incremented.
                        match (byte.is_ascii_whitespace(), is_word) {
                            // Ignore whitespace at the start of a line
                            (true, false) => {},
                            // Nothing to do while going through a word's chars
                            (false, true) => {},
                            // We've found a non-whitespace char
                            (false, false) => is_word = true,
                            // Found a whitespace char after a string of non-ws
                            (true, true) => {
                                self.words += 1;
                                is_word = false;
                            },
                        }
                    });

                    continue;
                },
                Err(e) => return Err(e.into()),
            }
        }

        Ok(())
    }

    pub fn print_results(&self, filename: &str, chars: bool, bytes: bool, lines: bool, words: bool) {
        print!(" ");

        // Printing chars over bytes is wc's behavior
        if chars {
            print!("  {} ", self.chars);
        } else if bytes {
            print!("  {} ", self.bytes);
        }
        if lines {
            print!("  {} ", self.lines);
        }
        if words {
            print!("  {} ", self.words);
        }

        if !chars && !bytes && !words && !lines {
            print!("  {} ", self.lines);
            print!("  {} ", self.words);
            print!("  {} ", self.bytes);
        }

        print!("{}\n", filename);
    }
}
