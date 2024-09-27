use std::{fs::read_to_string, io::Error};

#[derive(Default)]
pub struct Buffer {
    pub lines: Vec<String>,
}

impl Buffer {
    pub fn load(file_path: &str) -> Result<Self, Error> {
        let file_contents = read_to_string(file_path)?;
        let mut lines = Vec::new();
        for line in file_contents.lines() {
            lines.push(String::from(line));
        }
        Ok(Self {lines})
    }
    pub fn is_empty(&self) -> bool {
        self.lines.is_empty()
    }
}