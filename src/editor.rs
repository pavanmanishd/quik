use std::{cmp::min, io::Error, panic::{set_hook, take_hook},env};
use crossterm::event::{read, Event::{self, Key, Resize}, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
mod terminal;
use terminal::{Terminal,Size,Position};
mod view;
use view::View;

#[derive(Copy,Clone,Default)]
pub struct Location {
    pub x: usize,
    pub y: usize
}

pub struct Editor {
    should_quit: bool,
    location: Location,
    view: View
}

impl Editor {
    pub fn new() -> Result<Self, Error> {
        let current_hook = take_hook();
        set_hook(Box::new(move | panic_info | {
            let _ = Terminal::terminate();
            current_hook(panic_info);
        }));
        Terminal::intialize()?;
        let mut view = View::default();
        let args: Vec<String> = env::args().collect();
        if let Some(file_path) = args.get(1) {
            view.load(file_path);
        }

        Ok(Self {
            should_quit: false,
            location: Location::default(),
            view,
        })
    }
    pub fn run(&mut self) {
        loop {
            self.refresh_screen();
            if self.should_quit {
                break;
            }
            match read() {
                Ok(event) => self.evaluate_event(event),
                Err(err) => {
                    #[cfg(debug_assertions)]
                    {
                        panic!("Could not read event: {err:?}");
                    }
                }
            }
        }
    }

    fn move_point(&mut self, key_code: KeyCode) {
        let Location { mut x, mut y } = self.location;
        let Size { width, height } = Terminal::size().unwrap_or_default();
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
    }


    fn evaluate_event(&mut self, event: Event){
        match event {
            Key(KeyEvent {
                code, modifiers, kind: KeyEventKind::Press, ..
            }) => {
                match (code,modifiers) {
                    (KeyCode::Char('q') , KeyModifiers::CONTROL) => {
                        self.should_quit = true;
                    },
                    (KeyCode::Up
                    | KeyCode::Down
                    | KeyCode::Left
                    | KeyCode::Right
                    | KeyCode::PageDown
                    | KeyCode::PageUp
                    | KeyCode::End
                    | KeyCode::Home, _ ) => {
                        self.move_point(code);
                    },
                    _ => {},
                }
            },
            Resize(width_u16, height_u16) => {
                #[allow(clippy::as_conversions)]
                let height = height_u16 as usize;
                #[allow(clippy::as_conversions)]
                let width = width_u16 as usize;
                self.view.resize(Size {width,height});
            },
            _ => {}
        }
    }

    fn refresh_screen(&mut self) {
        let _ = Terminal::hide_cursor();
        self.view.render();
        let _ = Terminal::move_cursor_to(Position {
            x: self.location.x,
            y: self.location.y,
        });
        let _ = Terminal::show_cursor();
        let _ = Terminal::execute();
    }  
}


impl Drop for Editor {
    fn drop(&mut self) {
        let _ = Terminal::terminate();
        if self.should_quit {
            let _ = Terminal::print("Good Bye.\r\n");
        }
    }
}