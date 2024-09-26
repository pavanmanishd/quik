use crate::editor::terminal::{Size,Terminal,Position};
use std::io::Error;
const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct View {}

impl View {
    pub fn render() -> Result<(), Error> {
        let Size{height, ..} = Terminal::size()?;
        for i in 0..height {
            // Terminal::move_cursor_to(Position { x:0, y:i })?;
            Terminal::clear_line()?;
            Terminal::print("~")?;
            if i+1 < height {
                Terminal::print("\r\n")?;
            }
        }
        Self::welcome()?;
        Ok(())
    }

    fn welcome() -> Result<(), Error> {
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