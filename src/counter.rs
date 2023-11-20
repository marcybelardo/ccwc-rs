use anyhow::Result;
use std::io::BufRead;

pub struct Counter {
    pub contents: String,
    bytes: usize,
    lines: usize,
    words: usize,
    chars: usize,
}

impl Counter {
    pub fn new() -> Self {
        Self {
            contents: String::new(),
            bytes: 0,
            lines: 0,
            words: 0,
            chars: 0,
        }
    }

    pub fn load<R: BufRead>(&mut self, reader: &mut R) -> Result<()> {
        loop {
            match reader.read_line(&mut self.contents) {
                Ok(len) if len == 0 => break,
                Ok(len) => {
                    self.bytes += len;
                    self.lines += 1;

                    let mut is_word = false;
                    
                    self.contents.chars().for_each(|c| {
                        // We're not really using chars right now
                        // self.chars += 1;

                        match (c.is_whitespace(), is_word) {
                            (true, false) => {},
                            (false, true) => {},
                            (false, false) => is_word = true,
                            (true, true) => {
                                self.words += 1;
                                is_word = false;
                            },
                        }
                    });

                    self.contents.clear();
                    continue;
                },
                Err(e) => return Err(e.into()),
            }
        }

        Ok(())
    }

    pub fn bytes(&self) -> usize {
        self.bytes
    }

    pub fn lines(&self) -> usize {
        self.lines
    }

    pub fn words(&self) -> usize {
        self.words
    }

    pub fn chars(&self) -> usize {
        self.chars
    }
}
