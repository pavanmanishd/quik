use std::{cmp::min, io::Error};
use crossterm::event::{read, Event::{self, Key}, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
mod terminal;
use terminal::{Terminal,Size,Position};
mod view;
use view::View;


#[derive(Default)]
pub struct Editor {
    should_quit: bool,
    is_default: bool,
    location: Location,
    view: View
}

#[derive(Copy,Clone,Default)]
pub struct Location {
    pub x: usize,
    pub y: usize
}


impl Editor {
    pub fn run(&mut self) {
        Terminal::intialize().unwrap();
        let args: Vec<String> = std::env::args().collect();
        let file_path = args.get(1);
        let status: Result<(), Error> = self.view.load(file_path);
        match status {
            Ok(()) => (),
            Err(_err) => self.is_default = true,
        }
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
            self.evaluate_event(&event)?;
        }
        Ok(())
    }

    fn move_point(&mut self, key_code: KeyCode) -> Result<(), Error> {
        let Location { mut x, mut y } = self.location;
        let Size { width, height } = Terminal::size()?;
        match key_code {
            KeyCode::Up => {
                y = y.saturating_sub(1);
            },
            KeyCode::Down => {
                y = min(height.saturating_sub(1), y.saturating_add(1));
            },
            KeyCode::Left => {
                x = x.saturating_sub(1);
            },
            KeyCode::Right => {
                x = min(width.saturating_sub(1), x.saturating_add(1));
            },
            KeyCode::PageUp => {
                y = 0;
            },
            KeyCode::PageDown => {
                y = height.saturating_sub(1);
            },
            KeyCode::Home => {
                x = 0;
            },
            KeyCode::End => {
                x = width.saturating_sub(1);
            },
            _ => (),
        }
        self.location = Location{x,y};
        Ok(())
    }

    fn evaluate_event(&mut self, event: &Event) -> Result<(), Error> {
        if let Key(KeyEvent {
            code, modifiers, kind: KeyEventKind::Press, ..
        }) = event {
            match code {
                KeyCode::Char('q') if *modifiers == KeyModifiers::CONTROL => {
                    self.should_quit = true;
                },
                KeyCode::Up
                | KeyCode::Down
                | KeyCode::Left
                | KeyCode::Right
                | KeyCode::PageDown
                | KeyCode::PageUp
                | KeyCode::End
                | KeyCode::Home => {
                    self.move_point(*code)?;
                },
                _ => (),
            }
        }
        Ok(())
    }

    fn refresh_screen(&self) -> Result<(), Error> {
        Terminal::hide_cursor()?;
        Terminal::move_cursor_to(Position::default())?;
        if self.should_quit {
            Terminal::clear_screen()?;
            Terminal::print("Good Bye!\r\n")?;
        } else {
            self.view.render()?;
            if self.is_default {
                View::welcome()?;
            }
            Terminal::move_cursor_to(Position { x: self.location.x, y: self.location.y})?;
        }
        Terminal::show_cursor()?;
        Terminal::execute()?;
        Ok(())
    }  

}