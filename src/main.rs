extern crate termion;

use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use std::io::{self, Write};
use std::sync::mpsc;
use std::thread;
use std::time;

enum Event {
    Tick,
    Quit,
}

fn main() {
    let stdin = io::stdin();
    let mut stdout = io::stdout().into_raw_mode().unwrap();

    write!(stdout, "{}{}",
           termion::clear::All,
           termion::cursor::Goto(1, 1)).unwrap();
    stdout.flush().unwrap();


    let (tx, rx) = mpsc::channel();

    let tx2 = tx.clone();
    thread::spawn(move || {
        for c in stdin.keys() {
            match c.unwrap() {
                Key::Char('q') => tx2.send(Event::Quit).unwrap(),
                _ => {},
            }
        }
    });

    thread::spawn(move || {
        loop {
            tx.send(Event::Tick).unwrap();
            thread::sleep(time::Duration::from_millis(500));
        }
    });

    for event in rx {
        match event {
            Event::Tick => {
                write!(stdout, "Tick").unwrap();
                stdout.flush().unwrap();
            },
            Event::Quit => break,
        }
    }
}
