use std::{fs::read_to_string, io::Error};

pub struct Buffer {
    pub lines: Vec<String>,
}

impl Default for Buffer {
    fn default() -> Self {
        Self {
            lines: Vec::default(),
        }
    }
}

impl Buffer {
    pub fn load(&mut self, file_path: &str) -> Result<(), Error> {
        let file_contents = read_to_string(file_path)?;
        for line in file_contents.lines() {
            self.lines.push(String::from(line));
        }
        Ok(())
    }
}