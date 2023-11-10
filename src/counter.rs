use anyhow::Result;
use std::{fs::File, io::{Read, BufRead}};

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

    pub fn load_from_file(&mut self, file: &str) -> Result<()> {
        let mut file = File::open(file)?;
        file.read_to_string(&mut self.contents)?;

        Ok(())
    }

    pub fn load<R: BufRead>(&mut self, reader: &mut R) -> Result<()> {
        loop {
            match reader.read_line(&mut self.contents) {
                Ok(bytes_read) => {
                    self.bytes += bytes_read;
                    self.lines += 1;
                    Self::count_words(self)?;
                    Self::count_chars(self)?;
                },
                Err(e) => return Err(e.into()),
            }

            self.contents.clear();
        }
    }

    pub fn count_bytes(&mut self) -> Result<()> {
        self.bytes = self.contents.as_bytes().iter().count();

        Ok(())
    }

    pub fn bytes(&self) -> usize {
        self.bytes
    }

    pub fn count_lines(&mut self) -> Result<()> {
        self.lines = self.contents.chars().filter(|c| *c == '\n').count();

        Ok(())
    }

    pub fn lines(&self) -> usize {
        self.lines
    }

    pub fn count_words(&mut self) -> Result<()> {
        self.words = self
            .contents
            .chars()
            .collect::<Vec<char>>()
            .windows(2)
            .filter(|&pair| pair[0].is_whitespace() && !pair[1].is_whitespace())
            .count()
            // the iterator misses the first word in the text, so we add 1
            + 1;

        Ok(())
    }

    pub fn words(&self) -> usize {
        self.words
    }

    pub fn count_chars(&mut self) -> Result<()> {
        self.chars = self.contents.chars().count();

        Ok(())
    }

    pub fn chars(&self) -> usize {
        self.chars
    }
}
