extern crate termion;

use termion::raw::IntoRawMode;
use std::io::{self, Write};

use game;

pub fn init() -> Screen {
    let mut stdout = io::stdout().into_raw_mode().unwrap();
    write!(stdout, "{}", termion::cursor::Hide).unwrap();
    stdout.flush().unwrap();

    Screen { stdout }
}

pub struct Screen {
    stdout: termion::raw::RawTerminal<io::Stdout>,
}

impl Screen {
    pub fn clear(&mut self) {
        write!(self.stdout, "{}", termion::clear::All).unwrap();
    }

    pub fn flush(&mut self) {
        self.stdout.flush().unwrap();
    }

    pub fn reset(&mut self) {
        write!(self.stdout, "{}", termion::cursor::Show).unwrap();
        self.flush();
    }
}

impl game::Renderer for Screen {
    fn put_cell(&mut self, x: u16, y: u16, c: char) {
        write!(self.stdout, "{}{}",
               termion::cursor::Goto(x + 1, y + 1), c).unwrap();
    }
}

