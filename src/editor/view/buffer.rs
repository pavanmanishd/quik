use std::fs::{read_to_string, File};
use std::io::Error;
use std::io::Write;

use super::line::Line;
use super::Location;

#[derive(Default)]
pub struct Buffer {
    pub lines: Vec<Line>,
    pub file_name: Option<String>,
}

impl Buffer {
    pub fn load(file_name: &str) -> Result<Self, Error> {
        let contents = read_to_string(file_name)?;
        let mut lines = Vec::new();
        for value in contents.lines() {
            lines.push(Line::from(value));
        }
        Ok(Self { 
            lines,
            file_name: Some(file_name.to_string()),
        })
    }
    pub fn save(&self) -> Result<(), Error> {
        if let Some(file_name) = &self.file_name {
            let mut file = File::create(file_name)?;
            for line in &self.lines {
                writeln!(file, "{line}")?;
            }
        }
        Ok(())
    } 
    pub fn is_empty(&self) -> bool {
        self.lines.is_empty()
    }
    pub fn height(&self) -> usize {
        self.lines.len()
    }
    pub fn insert_char(&mut self, character: char, at: Location) {
        if at.line_index > self.lines.len() {
            return;
        }

        if at.line_index == self.lines.len() {
            self.lines.push(Line::from(&character.to_string()));
        } else if let Some(line) = self.lines.get_mut(at.line_index) {
            line.insert_char(character, at.grapheme_index);
        }
    }
    pub fn delete(&mut self, at: Location) {
        if let Some(line) = self.lines.get(at.line_index) {
            if at.grapheme_index >= line.grapheme_count() && self.lines.len() > at.line_index.saturating_add(1) {
                let next_line = self.lines.remove(at.line_index.saturating_add(1));
                self.lines[at.line_index].append(&next_line);
            } else if at.grapheme_index < line.grapheme_count() {
                self.lines[at.line_index].delete(at.grapheme_index);
            }
        }
    }
    pub fn insert_newline(&mut self, at: Location) {
        if at.line_index == self.height() {
            self.lines.push(Line::default());
        } else if let Some(line) = self.lines.get_mut(at.line_index) {
            let new = line.split(at.grapheme_index);
            self.lines.insert(at.line_index.saturating_add(1), new);
        }
    }
}