use crate::editor::terminal::{Size,Terminal,Position};
use std::io::{Error,ErrorKind};
use std::cmp::min;
const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");
mod buffer;
use buffer::Buffer;

#[derive(Default)]
pub struct View {
    buffer: Buffer,
    pub needs_redraw: bool
}

impl View {
    pub fn render(&mut self) -> Result<(), Error> {
        let Size { height, width } = Terminal::size()?;
        if self.needs_redraw {
            for row in 0..height {
                Terminal::clear_line()?;
                
                if let Some(buffer_line) = self.buffer.lines.get(row) {
                    let display_length = min(buffer_line.len(), width);
                    let visible_slice = buffer_line.get(0..display_length);
                    
                    if let Some(line_segment) = visible_slice {
                        Terminal::print(line_segment)?;
                    }
                    Terminal::print("\r\n")?;
                    continue;
                }
                
                Terminal::print("~")?;
                
                if row + 1 < height {
                    Terminal::print("\r\n")?;
                }
            }
        }
        self.needs_redraw = false;
        Ok(())
    }
    

    pub fn load(&mut self, file_path: Option<&String>) -> Result<(), Error> {
        if let Some(path) = file_path {
            self.buffer.load(path)?;
        } else {
            return Err(Error::new(ErrorKind::NotFound, "No file path provided"));
        }
        Ok(())
    }

    pub fn welcome() -> Result<(), Error> {
        let Size { width, height } = Terminal::size()?;
        let message = format!("{NAME} editor -- version {VERSION}");
        let y = height / 3;
        
        // Use saturating_sub to prevent underflow
        let x = width.saturating_sub(message.len()) / 2;
        
        Terminal::move_cursor_to(Position { x, y })?;
        Terminal::print(&message)?;
        Ok(())
    }   
}