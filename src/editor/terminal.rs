use crossterm::cursor::{Hide, MoveTo, Show};
use crossterm::style::Print;
use std::io::{stdout, Error, Write};
use crossterm::{queue, Command};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, size, Clear, ClearType};

pub struct Terminal {}

#[derive(Copy, Clone)]
pub struct Size {
    pub width: usize,
    pub height: usize
}

#[derive(Copy, Clone, Default)]
pub struct Position {
    pub x: usize,
    pub y: usize
}

impl Terminal {
    pub fn terminate() -> Result<(), Error> {
        Self::execute()?;
        disable_raw_mode()?;
        Ok(())
    }

    pub fn intialize() -> Result<(), Error> {
        enable_raw_mode()?;
        Self::clear_screen()?;
        Self::move_cursor_to(Position { x:0, y:0 })?;
        Self::execute()?;
        Ok(())
    }

    pub fn clear_screen() -> Result<(), Error> {
        Self::queue_command(Clear(ClearType::All))?;
        Ok(())
    }

    pub fn clear_line() -> Result<(), Error> {
        Self::queue_command(Clear(ClearType::CurrentLine))?;
        Ok(())
    }

    pub fn move_cursor_to(position: Position) -> Result<(), Error> {
        Self::queue_command(MoveTo(position.x as u16,position.y as u16))?;
        Ok(())
    }

    pub fn size() -> Result<Size, Error> {
        let (width, height) = size()?;
        Ok(Size {width: width as usize,height: height as usize})
    }

    pub fn hide_cursor() -> Result<(), Error> {
        Self::queue_command(Hide)?;
        Ok(())
    }

    pub fn show_cursor() -> Result<(), Error> {
        Self::queue_command(Show)?;
        Ok(())
    }

    pub fn print(data: &str) -> Result<(), Error> {
        Self::queue_command(Print(data))?;
        Ok(())
    }

    pub fn execute() -> Result<(), Error> {
        stdout().flush()?;
        Ok(())
    }

    pub fn queue_command<T: Command>(command: T) -> Result<(), Error> {
        queue!(stdout(), command)?;
        Ok(())
    }

}