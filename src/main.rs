extern crate termion;

mod game;
mod screen;

use termion::event::Key;
use termion::input::TermRead;
use std::io;
use std::sync::mpsc;
use std::thread;
use std::time;

use game::Event;

const WIDTH: usize = 20;
const HEIGHT: usize = 8;

fn main() {
    let stdin = io::stdin();

    let (tx, rx) = mpsc::channel();

    let tx2 = tx.clone();
    thread::spawn(move || {
        for c in stdin.keys() {
            match c.unwrap() {
                Key::Up        => tx2.send(Event::Up).unwrap(),
                Key::Down      => tx2.send(Event::Down).unwrap(),
                Key::Left      => tx2.send(Event::Left).unwrap(),
                Key::Right     => tx2.send(Event::Right).unwrap(),
                Key::Char('q') => tx2.send(Event::Quit).unwrap(),
                _ => {},
            }
        }
    });

    thread::spawn(move || {
        loop {
            tx.send(Event::Tick).unwrap();
            thread::sleep(time::Duration::from_millis(200));
        }
    });

    let mut screen = screen::init(WIDTH, HEIGHT);
    let mut game = game::init();

    for event in rx {
        match event {
            Event::Tick => {
                game.handle_event();
                game.update();

                screen.clear();
                game.render(&mut screen);
                screen.flush();
            },
            Event::Quit => break,
            e => {
                game.push_event(e);
            },
        }
    }
}
