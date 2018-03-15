extern crate termion;

use termion::raw::IntoRawMode;
use std::io::{self, Write};

use game;

pub fn init(width: usize, height: usize) -> Screen {
    let mut stdout = io::stdout().into_raw_mode().unwrap();
    write!(stdout, "{}{}",
           termion::clear::All,
           termion::cursor::Hide).unwrap();
    stdout.flush().unwrap();

    Screen {
        stdout,
        buffer: vec![vec![' '; width]; height],
    }
}

pub struct Screen {
    stdout: termion::raw::RawTerminal<io::Stdout>,
    buffer: Vec<Vec<char>>,
}

impl Screen {
    pub fn clear(&mut self) {
        for row in self.buffer.iter_mut() {
            for cell in row.iter_mut() {
                *cell = ' ';
            }
        }
    }

    pub fn flush(&mut self) {
        write!(self.stdout, "{}", termion::cursor::Goto(1, 1)).unwrap();
        for row in self.buffer.iter() {
            write!(self.stdout, "{}\n\r", row.iter().collect::<String>()).unwrap();
        }
        self.stdout.flush().unwrap();
    }
}

impl Drop for Screen {
    fn drop(&mut self) {
        write!(self.stdout, "{}", termion::cursor::Show).unwrap();
        self.stdout.flush().unwrap();
    }
}

impl game::Renderer for Screen {
    fn put_cell(&mut self, x: u16, y: u16, c: char) {
        self.buffer[y as usize][x as usize] = c;
    }
}

