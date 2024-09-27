use crate::editor::terminal::{Size,Terminal};
const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");
mod buffer;
use buffer::Buffer;

pub struct View {
    buffer: Buffer,
    needs_redraw: bool,
    size: Size
}

impl View {
    pub fn resize(&mut self, to:Size) {
        self.size = to;
        self.needs_redraw = true;
    }

    fn render_line(at: usize, line_text: &str) {
        let result = Terminal::print_row(at, line_text);
        debug_assert!(result.is_ok(), "Failed to render line");
    }

    pub fn render(&mut self) {
        if !self.needs_redraw {
            return;
        }
        let Size { height, width } = self.size;
        if height == 0 || width == 0 {
            return;
        }

        #[allow(clippy::integer_division)]
        let vertical_center = height/3;

        for row in 0..height {
            if let Some(buffer_line) = self.buffer.lines.get(row) {
                let truncated_line = if buffer_line.len() >= width {
                    &buffer_line[0..width]
                } else {
                    buffer_line
                };
                Self::render_line(row, truncated_line);
            } else if row == vertical_center && self.buffer.is_empty() {
                Self::render_line(row, &Self::build_welcome_message(width));
            } else {
                Self::render_line(row, "~");
            }
        }
        self.needs_redraw = false;
    }
    
    pub fn build_welcome_message(width: usize) -> String {
        if width == 0 {
            return " ".to_string();
        }
        let message = format!("{NAME} editor -- version {VERSION}");
        let len = message.len();
        if width <= len {
            return "~".to_string();
        }

        #[allow(clippy::integer_division)]
        let padding = (width.saturating_sub(len).saturating_sub(1)) /2;
        
        let mut full_message = format!("~{}{}"," ".repeat(padding),message);
        full_message.truncate(width);
        full_message
    }

    pub fn load(&mut self, file_path: &str) {
        if let Ok(buffer) = Buffer::load(file_path) {
            self.buffer = buffer;
            self.needs_redraw = true;
        }        
    }

}


impl Default for View {
    fn default() -> Self {
        Self { 
            buffer: Buffer::default(), 
            needs_redraw: true, 
            size: Terminal::size().unwrap_or_default(), 
        }
    }
}