use unicode_segmentation::UnicodeSegmentation;
use std::{cmp, ops::Range};

pub struct Line {
    string: String,
}

impl Line {
    pub fn from(line_str: &str) -> Self {
        Self {
            string: String::from(line_str),
        }
    }
    pub fn get(&self, range: Range<usize>) -> String {
        let start = range.start;
        let end = cmp::min(range.end, self.string.len());
        // self.string.get(start..end).unwrap_or_default().to_string()
        self.string.graphemes(true).skip(start).take(end.saturating_sub(start)).collect()
    }
    pub fn len(&self) -> usize {
        self.string[..].graphemes(true).count()
    }
}