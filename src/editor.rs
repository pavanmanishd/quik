use std::io::Error;
use crossterm::event::{read, Event::{self, Key}, KeyCode::Char, KeyEvent, KeyEventKind, KeyModifiers};
mod terminal;
use terminal::{Terminal,Size,Position};

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


    fn repl(&mut self) -> Result<(), Error> {
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
            code, modifiers, kind: KeyEventKind::Press, ..
        }) = event {
            match code {
                Char('q') if *modifiers == KeyModifiers::CONTROL => {
                    self.should_quit = true;
                },
                _ => (),
            }
        }
    }

    fn refresh_screen(&self) -> Result<(), Error> {
        Terminal::hide_cursor()?;
        if self.should_quit {
            Terminal::clear_screen()?;
            Terminal::print("Good Bye!\r\n")?;
        } else {
            Self::draw_rows()?;
            Self::welcome()?;
            Terminal::move_cursor_to(Position { x: 0, y: 0})?;
        }
        Terminal::show_cursor()?;
        Terminal::execute()?;
        Ok(())
    }

    fn draw_rows() -> Result<(), Error> {
        let Size{height, ..} = Terminal::size()?;
        for i in 0..height {
            // Terminal::move_cursor_to(Position { x:0, y:i })?;
            Terminal::clear_line()?;
            Terminal::print("~")?;
            if i+1 < height {
                Terminal::print("\r\n")?;
            }
        }
        Ok(())
    }

    fn welcome() -> Result<(), Error> {
        let Size { width, height } = Terminal::size()?;
        let message = "Welcome to Quik v0.0";
        let y = height / 3;
        
        // Use saturating_sub to prevent underflow
        let x = width.saturating_sub(message.len() as u16) / 2;
        
        Terminal::move_cursor_to(Position { x, y })?;
        Terminal::print(message)?;
        Ok(())
    }    

}