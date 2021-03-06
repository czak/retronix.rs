extern crate termion;

use termion::raw::IntoRawMode;
use termion::screen::AlternateScreen;
use std::io::{self, Write};
use std::fmt;
use renderer::{Renderer, Color};

pub fn init(width: usize, height: usize) -> Screen {
    let mut stdout = AlternateScreen::from(io::stdout().into_raw_mode().unwrap());
    write!(stdout, "{}{}",
           termion::clear::All,
           termion::cursor::Hide).unwrap();
    stdout.flush().unwrap();

    Screen {
        stdout,
        buffer: vec![vec![(' ', Color::White); width]; height],
    }
}

pub struct Screen {
    stdout: termion::screen::AlternateScreen<termion::raw::RawTerminal<io::Stdout>>,
    buffer: Vec<Vec<(char, Color)>>,
}

impl Screen {
    pub fn clear(&mut self) {
        for row in self.buffer.iter_mut() {
            for cell in row.iter_mut() {
                *cell = (' ', Color::White);
            }
        }
    }

    pub fn flush(&mut self) {
        let (cols, rows) = termion::terminal_size().unwrap();
        write!(self.stdout, "{}", termion::cursor::Goto(1, 1)).unwrap();
        for row in self.buffer.iter().take((rows - 1) as usize) {
            for &(c, color) in row.iter().take(cols as usize) {
                write!(self.stdout, "{}{}",
                       color,
                       c).unwrap();
            }
            write!(self.stdout, "\n\r").unwrap();
        }
        self.stdout.flush().unwrap();
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let termion_color = match *self {
            Color::White => format!("{}", termion::color::Fg(termion::color::White)),
            Color::Cyan => format!("{}", termion::color::Fg(termion::color::Cyan)),
            Color::Magenta => format!("{}", termion::color::Fg(termion::color::Magenta)),
        };
        write!(f, "{}", termion_color)
    }
}

impl Drop for Screen {
    fn drop(&mut self) {
        write!(self.stdout, "{}", termion::cursor::Show).unwrap();
        self.stdout.flush().unwrap();
    }
}

impl Renderer for Screen {
    fn put_cell(&mut self, x: u16, y: u16, c: char, color: Color) {
        self.buffer[y as usize][x as usize] = (c, color);
    }
}

