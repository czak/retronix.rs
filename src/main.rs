extern crate termion;

use termion::event::Key;
use termion::input::TermRead;
use std::io;
use std::sync::mpsc;
use std::thread;
use std::time;

mod game;
mod terminal;

enum Event {
    Tick,
    Quit,
}

fn main() {
    let stdin = io::stdin();

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

    let mut terminal = terminal::init();
    let mut game = game::init();

    for event in rx {
        match event {
            Event::Tick => {
                game.update();

                terminal.clear();
                game.render(&mut terminal);
                terminal.flush();
            },
            Event::Quit => break,
        }
    }

    terminal.reset();
}
