extern crate termion;

use termion::raw::IntoRawMode;
use std::io::{self, Write};

use game;

pub fn init() -> Terminal {
    Terminal {
        stdout: io::stdout().into_raw_mode().unwrap(),
    }
}

pub struct Terminal {
    stdout: termion::raw::RawTerminal<io::Stdout>,
}

impl Terminal {
    pub fn clear(&mut self) {
        write!(self.stdout, "{}", termion::clear::All).unwrap();
        self.flush();
    }

    pub fn flush(&mut self) {
        self.stdout.flush().unwrap();
    }
}

impl game::Renderer for Terminal {
    fn put_cell(&mut self, x: u16, y: u16, c: char) {
        write!(self.stdout, "{}{}",
               termion::cursor::Goto(x, y), c).unwrap();
    }
}

