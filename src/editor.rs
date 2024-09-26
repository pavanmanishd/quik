use std::io::stdout;
use crossterm::{cursor::{Hide, Show}, event::{read, Event::{self, Key}, KeyCode::Char, KeyEvent, KeyModifiers}, execute, style::Print};
mod terminal;
use terminal::Terminal;

pub struct Editor {
    should_quit: bool,
}


impl Editor {
    pub const  fn default() -> Self {
        Self { should_quit: false }
    }

    pub fn run(&mut self) {
        Terminal::intialize().unwrap();
        let result = self.repl();
        Terminal::terminate().unwrap();
        result.unwrap();
    }


    fn repl(&mut self) -> Result<(), std::io::Error> {
        loop {
            self.refresh_screen()?;
            if self.should_quit {
                break;
            }
            let event = read()?;
            self.evaluate_event(&event);
        }
        Ok(())
    }

    fn evaluate_event(&mut self, event: &Event) {
        if let Key(KeyEvent {
            code, modifiers, ..
        }) = event {
            match code {
                Char('q') if *modifiers == KeyModifiers::CONTROL => {
                    self.should_quit = true;
                },
                _ => (),
            }
        }
    }

    fn refresh_screen(&self) -> Result<(), std::io::Error> {
        execute!(stdout(), Hide)?;
        if self.should_quit {
            Terminal::clear_screen()?;
            print!("Good Bye!\r\n");
        } else {
            Self::draw_rows()?;
            Terminal::move_cursor_to(0,0)?;
        }
        execute!(stdout(), Show)?;
        Ok(())
    }

    fn draw_rows() -> Result<(), std::io::Error> {
        let size = Terminal::size()?;
        let row_size = size.1;
        for i in 0..row_size {
            Terminal::move_cursor_to(0, i)?;
            Terminal::clear_row()?;
            // print!("~");
            execute!(stdout(), Print("~"))?;
        }
        Ok(())
    }

}