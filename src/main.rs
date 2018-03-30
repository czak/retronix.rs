extern crate termion;
extern crate rand;

mod game;
mod screen;

use termion::event::Key;
use termion::input::TermRead;
use std::io;
use std::sync::mpsc;
use std::thread;
use std::time;

use game::Event;

const WIDTH: usize = 80;
const HEIGHT: usize = 26;

#[allow(dead_code)]
struct FakeScreen {}

#[allow(dead_code)]
impl FakeScreen {
    fn clear(&mut self) {}
    fn flush(&mut self) {}
}

impl game::Renderer for FakeScreen {
    fn put_cell(&mut self, _x: u16, _y: u16, _c: char) {}
}

fn main() {
    let (tx, rx) = mpsc::channel();

    input_thread(tx.clone());
    tick_thread(tx.clone());

    let mut screen = screen::init(WIDTH, HEIGHT);
    // let mut screen = FakeScreen {};
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

fn input_thread(tx: mpsc::Sender<Event>) {
    thread::spawn(move || {
        for c in io::stdin().keys() {
            match c.unwrap() {
                Key::Up        => tx.send(Event::Up).unwrap(),
                Key::Down      => tx.send(Event::Down).unwrap(),
                Key::Left      => tx.send(Event::Left).unwrap(),
                Key::Right     => tx.send(Event::Right).unwrap(),
                Key::Char('q') => tx.send(Event::Quit).unwrap(),
                _ => {},
            }
        }
    });
}

fn tick_thread(tx: mpsc::Sender<Event>) {
    thread::spawn(move || {
        loop {
            tx.send(Event::Tick).unwrap();
            thread::sleep(time::Duration::from_millis(100));
        }
    });

}
